import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, Computed, makeObservable, Observable, runInAction } from "@wirestate/mobx";
import { Texture } from "three";

import { assetsRawCommands } from "@/core/bindings/commands/assets-raw";
import { visualsCommands } from "@/core/bindings/commands/visuals";
import { visualsRawCommands } from "@/core/bindings/commands/visuals-raw";
import { AssetWorldSpec, SelectedVisualDescription, VisualSource } from "@/core/bindings/types/xrf-app";
import { transformError } from "@/core/error/lib";
import { releaseEditorProject } from "@/core/ipc/release";
import { emitNotification, ENotificationSeverity } from "@/core/notifications/lib";
import { EApplicationId } from "@/core/routing/application";
import { getProjectGamedataPath } from "@/core/settings/lib/path/project";
import { ProjectService } from "@/core/settings/services/project/project.service";
import { describeVisualSource } from "@/core/visuals/lib/visual-source";
import {
  createDdsTexture,
  EVisualTextureState,
  ILoadableTexture,
  IVisualTextureStatus,
  toInitialTextureState,
  toLoadableTextures,
} from "@/core/visuals/lib/visual-texture";
import { createVisualViews, IVisualModelViews } from "@/core/visuals/lib/visual-views";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

/** A visual that is open: what it is, where it came from, and the views the scene draws. */
export interface IOpenVisual {
  selected: SelectedVisualDescription;
  views: IVisualModelViews;
}

/**
 * The visual on screen.
 *
 * Loading is two calls by design - the description is typed and the geometry is raw bytes, and a tauri
 * command returns one or the other, never both. They are addressed by source rather than by what is
 * selected, so a response that arrives after the user moved on is discardable instead of being paired
 * with the wrong model.
 */
@Injectable()
export class VisualsService {
  public readonly log: Logger = new Logger(this.constructor.name);

  /** Distinguishes a response for the model being asked about from one the user already moved past. */
  private requestId: number = 0;

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public visual: Loadable<Nullable<IOpenVisual>> = createLoadable(null);

  /**
   * Uploaded textures by submesh index, for the viewport to apply.
   *
   * Textures reach the scene through state rather than by handing the service a scene reference: the scene is owned by
   * the component that mounts it, and a store that reached into webgl would have two owners for one context.
   */
  @Observable()
  public textures: ReadonlyMap<number, Texture> = new Map();

  /** What became of each submesh's texture, so a panel can report it rather than leaving a submesh unexplained. */
  @Observable()
  public textureStatuses: ReadonlyMap<number, IVisualTextureStatus> = new Map();

  /**
   * @returns The path or entry the open visual was read from, or null when nothing is open.
   */
  @Computed()
  public get sourceLabel(): Nullable<string> {
    const source: Nullable<VisualSource> = this.visual.value?.selected.source ?? null;

    return source ? describeVisualSource(source) : null;
  }

  /**
   * @returns The directory the open model sits in, or null when there is nothing to browse from.
   *
   * Only a loose file has one: an asset is already being browsed, and its bytes may sit inside a volume that no
   * directory contains.
   */
  @Computed()
  public get containingRoot(): Nullable<string> {
    const source: Nullable<VisualSource> = this.visual.value?.selected.source ?? null;

    if (source?.kind !== "file") {
      return null;
    }

    const separatorAt: number = Math.max(source.path.lastIndexOf("\\"), source.path.lastIndexOf("/"));

    return separatorAt > 0 ? source.path.slice(0, separatorAt) : null;
  }

  /**
   * @returns Whether the open model animates from anything, referenced or embedded.
   */
  @Computed()
  public get hasMotions(): boolean {
    const selected: Nullable<SelectedVisualDescription> = this.visual.value?.selected ?? null;

    return Boolean(selected && (selected.dependencies.motions.length || selected.description.embeddedMotions.length));
  }

  public constructor(
    private readonly eventBus: EventBus = inject(EventBus),
    private readonly projectService: ProjectService = inject(ProjectService)
  ) {
    makeObservable(this);
  }

  /**
   * Restore whatever the backend still has selected.
   *
   * A reload re-provisions this service, and the backend keeps the selection for exactly this reason, so
   * the viewer comes back showing the same model rather than an empty picker.
   */
  @OnProvision()
  public async onProvision(): Promise<void> {
    try {
      const selected: Nullable<SelectedVisualDescription> = await visualsCommands.getModel();

      if (selected) {
        this.log.info("Restoring selected visual:", describeVisualSource(selected.source));

        await this.loadGeometry(selected);
      }
    } catch (error) {
      this.log.error("Failed to restore selected visual:", error);
    } finally {
      runInAction(() => {
        this.isReady = true;
      });
    }
  }

  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(visualsCommands.closeModel);
  }

  /**
   * Open a loose visual from disk.
   *
   * @param path - Filesystem path of the `.ogf` file.
   */
  @BoundAction()
  public async openFile(path: string): Promise<void> {
    // Centred on the file, so its own tree and installation are searched for its textures - and searched again when
    // those textures are read, because the world travels with the description.
    await this.open({ kind: "file", path }, [], path);
  }

  /**
   * Open a visual of a browsed world, loose or archived alike.
   *
   * The roots come from the caller because the browsed root is what makes the asset addressable at all: opening
   * `meshes\wpn\wpn_ak74.ogf` means nothing without the world it names.
   *
   * @param logicalPath - Engine identity of the visual, as the listing reported it.
   * @param roots - Roots searched ahead of the project's own, usually the browsed one.
   */
  @BoundAction()
  public async openAsset(logicalPath: string, roots: Array<string>): Promise<void> {
    await this.open({ kind: "asset", logicalPath }, roots);
  }

  @BoundAction()
  public async close(): Promise<void> {
    runInAction(() => {
      this.requestId += 1;
      this.visual = createLoadable(null);
      this.releaseTextures();
      this.textureStatuses = new Map();
    });

    try {
      await visualsCommands.closeModel();
    } catch (error) {
      this.log.error("Failed to close visual:", error);
    }
  }

  /**
   * Load a visual and put it on screen.
   *
   * @param source - Visual source to open.
   * @param roots - Roots searched ahead of the project's own.
   * @param asset - Asset the world is centred on, whose own tree is searched first.
   */
  private async open(source: VisualSource, roots: Array<string> = [], asset: Nullable<string> = null): Promise<void> {
    this.log.info("Opening visual:", describeVisualSource(source));

    try {
      const request: number = runInAction(() => {
        this.requestId += 1;
        this.visual = this.visual.asLoading();

        return this.requestId;
      });

      const world: AssetWorldSpec = await this.getWorld(roots, asset);
      const selected: SelectedVisualDescription = await visualsCommands.openModel(source, world);

      await this.loadGeometry(selected, request);
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Open error:", transformed);

      runInAction(() => {
        this.visual = this.visual.asFailed(transformed, null);
      });

      emitNotification(this.eventBus, {
        details: `${describeVisualSource(source)}\n${transformed.message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.VISUALS_EXPLORER,
        title: "Could not open visual",
      });
    }
  }

  /**
   * Fetch and view the geometry of an already described visual.
   *
   * Throws rather than reporting, so the caller that started the open owns the failure and there is one
   * place that turns it into state.
   *
   * @param selected - Typed description and source returned by the backend.
   * @param request - Request identity used to discard stale geometry.
   */
  private async loadGeometry(selected: SelectedVisualDescription, request: number = ++this.requestId): Promise<void> {
    // The world the open used travels back with the description, so a geometry read after a reload searches what the
    // open searched rather than whatever the frontend would name now.
    const buffer: ArrayBuffer = await visualsRawCommands.readGeometry(selected.source, selected.world);

    if (request !== this.requestId) {
      this.log.info(
        "Discarding geometry for a visual that is no longer selected:",
        describeVisualSource(selected.source)
      );

      return;
    }

    const views: IVisualModelViews = createVisualViews(selected.description, buffer);

    runInAction(() => {
      this.visual = this.visual.asReady({ selected, views });
      this.releaseTextures();
      this.textureStatuses = new Map(
        selected.dependencies.textures.map((texture) => [
          texture.submeshIndex,
          { reason: null, state: toInitialTextureState(texture.resolution), submeshIndex: texture.submeshIndex },
        ])
      );
    });

    void this.loadTextures(selected, request);
  }

  /**
   * Fetch each located texture and apply it as it lands.
   *
   * @param selected - Open visual whose textures should be loaded.
   * @param request - Request identity used to discard stale textures.
   */
  private async loadTextures(selected: SelectedVisualDescription, request: number): Promise<void> {
    const loadable: Array<ILoadableTexture> = toLoadableTextures(selected.dependencies.textures);

    if (!loadable.length) {
      return;
    }

    this.log.info(`Loading ${loadable.length} textures for:`, describeVisualSource(selected.source));

    await Promise.all(loadable.map((texture) => this.loadTexture(texture, selected.world, request)));
  }

  /**
   * One texture, from bytes to an uploaded texture or to a stated reason it is not one.
   *
   * Read by the logical path the open already resolved, so the bytes come from the file the description named — a
   * substituted dummy included — rather than from a second lookup that could answer differently.
   *
   * @param texture - Submesh identity and the logical path resolution located.
   * @param world - The mounted world the asset is read from.
   * @param request - Request identity used to discard a late response.
   */
  private async loadTexture(texture: ILoadableTexture, world: AssetWorldSpec, request: number): Promise<void> {
    try {
      const bytes: ArrayBuffer = await assetsRawCommands.readAsset(world, texture.logicalPath);

      if (request !== this.requestId) {
        return;
      }

      const uploaded: Nullable<Texture> = createDdsTexture(bytes);

      runInAction(() => {
        if (uploaded) {
          this.textures = new Map(this.textures).set(texture.submeshIndex, uploaded);
        }

        this.setTextureStatus(texture.submeshIndex, {
          reason: null,
          state: uploaded ? EVisualTextureState.APPLIED : EVisualTextureState.UNSUPPORTED_FORMAT,
          submeshIndex: texture.submeshIndex,
        });
      });
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error(`Failed to load texture '${texture.logicalPath}':`, transformed);

      if (request !== this.requestId) {
        return;
      }

      runInAction(() => {
        this.setTextureStatus(texture.submeshIndex, {
          reason: transformed.message,
          state: EVisualTextureState.FAILED,
          submeshIndex: texture.submeshIndex,
        });
      });
    }
  }

  private setTextureStatus(submeshIndex: number, status: IVisualTextureStatus): void {
    this.textureStatuses = new Map(this.textureStatuses).set(submeshIndex, status);
  }

  /**
   * Free the uploaded textures of a model being replaced.
   *
   * The scene disposes what it was handed when its model changes, and this disposes what the store still holds, so a
   * texture is freed by whichever side outlives the other.
   */
  private releaseTextures(): void {
    for (const texture of this.textures.values()) {
      texture.dispose();
    }

    this.textures = new Map();
  }

  /**
   * The world a visual's references are searched in, after the visual's own tree.
   *
   * Only the frontend knows which project is configured, which is why the world is named on every call rather than
   * derived by the backend: it can derive the roots implied by an asset, but not an ambient one. Naming it rather than
   * holding a handle is also what lets a reload pick up where it left off, and another surface address the same assets.
   */
  private async getWorld(roots: Array<string> = [], asset: Nullable<string> = null): Promise<AssetWorldSpec> {
    const projectPath: Nullable<string> = this.projectService.xrfProjectPath;
    const project: Array<string> = projectPath ? [await getProjectGamedataPath(projectPath)] : [];

    // The caller's roots come first: a browsed tree is the nearer answer, and the project is the fallback behind it.
    return { asset, roots: [...roots, ...project] };
  }
}
