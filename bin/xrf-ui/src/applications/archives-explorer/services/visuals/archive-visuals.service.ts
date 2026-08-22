import { Injectable, OnDeactivation } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";
import { Texture } from "three";

import { assetsRawCommands } from "@/core/bindings/commands/assets-raw";
import { visualsCommands } from "@/core/bindings/commands/visuals";
import { visualsRawCommands } from "@/core/bindings/commands/visuals-raw";
import { AssetWorldSpec, SelectedVisualDescription, VisualSource } from "@/core/bindings/types/xrf-app";
import { transformError } from "@/core/error/lib";
import { releaseEditorProject } from "@/core/ipc/release";
import { createDdsTexture, ILoadableTexture, toLoadableTextures } from "@/core/visuals/lib/visual-texture";
import { createVisualViews, IVisualModelViews } from "@/core/visuals/lib/visual-views";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

/** A model previewed from an archive: what it is, and the views the scene draws. */
export interface IPreviewedVisual {
  selected: SelectedVisualDescription;
  views: IVisualModelViews;
}

/**
 * The model an archive entry is previewed as.
 *
 * Reads through the mounted asset world rather than through the archive project: an `.ogf` entry is addressed by its
 * engine identity, so the same commands serve it here and in the visuals explorer, and its textures resolve out of the
 * same volumes it came from.
 *
 * Deliberately thinner than the explorer's service - no picker, no browsed world, no project fallback. A preview
 * follows the tree selection and nothing else. What the two share, once both shapes have settled, is worth extracting;
 * guessing that seam before this existed would have been guessing.
 */
@Injectable()
export class ArchiveVisualsService {
  public readonly log: Logger = new Logger(this.constructor.name);

  /** Distinguishes a response for the entry being previewed from one the user already clicked past. */
  private requestId: number = 0;

  @Observable()
  public visual: Loadable<Nullable<IPreviewedVisual>> = createLoadable(null);

  /** Uploaded textures by submesh index, for the viewport to apply. */
  @Observable()
  public textures: ReadonlyMap<number, Texture> = new Map();

  public constructor() {
    makeObservable(this);
  }

  @OnDeactivation()
  public onDeactivation(): void {
    this.release();
    releaseEditorProject(visualsCommands.closeModel);
  }

  /**
   * Preview one archived model.
   *
   * @param root - Directory the archive project was opened at, which is the world the entry lives in.
   * @param logicalPath - Engine identity of the entry, as the archive records it.
   */
  @BoundAction()
  public async preview(root: string, logicalPath: string): Promise<void> {
    const world: AssetWorldSpec = { asset: null, roots: [root] };
    const source: VisualSource = { kind: "asset", logicalPath };

    this.log.info("Previewing archived visual:", logicalPath);

    const request: number = runInAction(() => {
      this.requestId += 1;
      this.visual = this.visual.asLoading();
      this.release();

      return this.requestId;
    });

    try {
      const selected: SelectedVisualDescription = await visualsCommands.openModel(source, world);
      const buffer: ArrayBuffer = await visualsRawCommands.readGeometry(source, selected.world);

      if (request !== this.requestId) {
        return;
      }

      const views: IVisualModelViews = createVisualViews(selected.description, buffer);

      runInAction(() => {
        this.visual = this.visual.asReady({ selected, views });
      });

      void this.loadTextures(selected, request);
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to preview archived visual:", transformed);

      if (request === this.requestId) {
        runInAction(() => {
          this.visual = this.visual.asFailed(transformed, null);
        });
      }
    }
  }

  /** Drop whatever is previewed, for a selection that is no longer a model. */
  @BoundAction()
  public clear(): void {
    runInAction(() => {
      this.requestId += 1;
      this.visual = createLoadable(null);
      this.release();
    });
  }

  /**
   * Fetch each located texture and apply it as it lands.
   *
   * @param selected - Model whose textures should be loaded.
   * @param request - Request identity used to discard textures for a model already clicked past.
   */
  private async loadTextures(selected: SelectedVisualDescription, request: number): Promise<void> {
    const loadable: Array<ILoadableTexture> = toLoadableTextures(selected.dependencies.textures);

    await Promise.all(
      loadable.map(async (texture: ILoadableTexture) => {
        try {
          const bytes: ArrayBuffer = await assetsRawCommands.readAsset(selected.world, texture.logicalPath);

          if (request !== this.requestId) {
            return;
          }

          const uploaded: Nullable<Texture> = createDdsTexture(bytes);

          if (uploaded) {
            runInAction(() => {
              this.textures = new Map(this.textures).set(texture.submeshIndex, uploaded);
            });
          }
        } catch (error: unknown) {
          this.log.error(`Failed to read texture '${texture.logicalPath}':`, transformError(error));
        }
      })
    );
  }

  /**
   * Free the uploaded textures of a model being replaced.
   *
   * The scene disposes what it was handed when its model changes, and this disposes what the service still holds, so a
   * texture is freed by whichever side outlives the other.
   */
  private release(): void {
    for (const texture of this.textures.values()) {
      texture.dispose();
    }

    this.textures = new Map();
  }
}
