import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, Computed, makeObservable, Observable, runInAction } from "@wirestate/mobx";
import { Texture } from "three";

import {
  SelectedVisualDescription,
  SubmeshTexture,
  commands as visualsCommands,
  VisualSource,
} from "@/core/bindings/xrf-app-visuals";
import { commands as visualsRawCommands } from "@/core/bindings/xrf-app-visuals-raw";
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
    await this.open({ kind: "file", path });
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
   */
  private async open(source: VisualSource): Promise<void> {
    this.log.info("Opening visual:", describeVisualSource(source));

    try {
      const request: number = runInAction(() => {
        this.requestId += 1;
        this.visual = this.visual.asLoading();

        return this.requestId;
      });

      const selected: SelectedVisualDescription = await visualsCommands.openModel(source, await this.getFallbackRoot());

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
        source: EApplicationId.VISUALS_VIEWER,
        title: "Could not open visual",
      });
    }
  }

  /**
   * Fetch and view the geometry of an already described visual.
   *
   * Throws rather than reporting, so the caller that started the open owns the failure and there is one
   * place that turns it into state.
   */
  private async loadGeometry(selected: SelectedVisualDescription, request: number = ++this.requestId): Promise<void> {
    const buffer: ArrayBuffer = await visualsRawCommands.readGeometry(selected.source);

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
        selected.textures.map((texture) => [
          texture.submeshIndex,
          { reason: null, state: toInitialTextureState(texture), submeshIndex: texture.submeshIndex },
        ])
      );
    });

    void this.loadTextures(selected, request);
  }

  /**
   * Fetch each located texture and apply it as it lands.
   */
  private async loadTextures(selected: SelectedVisualDescription, request: number): Promise<void> {
    const loadable: Array<SubmeshTexture & { reference: string }> = toLoadableTextures(selected.textures);

    if (!loadable.length) {
      return;
    }

    this.log.info(`Loading ${loadable.length} textures for:`, describeVisualSource(selected.source));

    const fallbackRoot: Nullable<string> = await this.getFallbackRoot();

    await Promise.all(loadable.map((texture) => this.loadTexture(selected.source, texture, fallbackRoot, request)));
  }

  /**
   * One texture, from bytes to an uploaded texture or to a stated reason it is not one.
   */
  private async loadTexture(
    source: VisualSource,
    texture: SubmeshTexture & { reference: string },
    fallbackRoot: Nullable<string>,
    request: number
  ): Promise<void> {
    try {
      const bytes: ArrayBuffer = await visualsRawCommands.readTexture(source, texture.reference, fallbackRoot);

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

      this.log.error(`Failed to load texture '${texture.reference}':`, transformed);

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
   * The root to fall back to when a visual's own tree does not answer.
   *
   * Only the frontend knows which project is configured, which is why the backend takes it as an argument rather than
   * deriving it: it can derive the per-visual root, but not an ambient one.
   */
  private async getFallbackRoot(): Promise<Nullable<string>> {
    const projectPath: Nullable<string> = this.projectService.xrfProjectPath;

    return projectPath ? getProjectGamedataPath(projectPath) : null;
  }
}
