import { Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, Computed, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { assetsCommands } from "@/core/bindings/commands/assets";
import { XrayAsset } from "@/core/bindings/types/xrf-vfs";
import { transformError } from "@/core/error/lib";
import { createLoadable, Loadable } from "@/lib/loadable";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

/** Where the browsed root is remembered, so a reload comes back to the tree it was showing. */
const ROOT_STORAGE_KEY: string = "xrf.visuals.browse-root";

/**
 * The world being browsed, and every visual in it.
 *
 * Separate from the service that owns the open model because the two have different lifetimes: a root outlives the
 * dozens of models opened under it, and a model can be open with no root at all.
 */
@Injectable()
export class VisualsBrowseService {
  public readonly log: Logger = new Logger(this.constructor.name);

  /** The root being browsed, or null when a single model was opened directly. */
  @Observable()
  public root: Nullable<string> = null;

  @Observable()
  public visuals: Loadable<Array<XrayAsset>> = createLoadable([]);

  /**
   * @returns Whether a root is open, which is what publishes the tree panel.
   */
  @Computed()
  public get isBrowsing(): boolean {
    return this.root !== null;
  }

  /**
   * @returns The roots an open searches ahead of the project's own.
   */
  @Computed()
  public get roots(): Array<string> {
    return this.root ? [this.root] : [];
  }

  public constructor() {
    makeObservable(this);
  }

  /**
   * Re-open whatever root was last browsed.
   *
   * A reload loses the tree but not the intent, and coming back to an empty panel beside a model that is still open
   * reads as a failure rather than a fresh start.
   */
  @OnProvision()
  public async onProvision(): Promise<void> {
    const stored: Nullable<string> = getLocalStorageValue(ROOT_STORAGE_KEY);

    if (stored) {
      this.log.info("Restoring browsed root:", stored);

      await this.openRoot(stored);
    }
  }

  /**
   * Forget the browsed root on the way out of the application.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    this.close();
  }

  /**
   * Browse a root and list every visual in it.
   *
   * @param root - Filesystem path of the directory or installation to browse.
   */
  @BoundAction()
  public async openRoot(root: string): Promise<void> {
    this.log.info("Browsing root:", root);

    runInAction(() => {
      this.root = root;
      this.visuals = this.visuals.asLoading();
    });

    setLocalStorageValue(ROOT_STORAGE_KEY, root);

    try {
      const visuals: Array<XrayAsset> = await assetsCommands.listAssets({ asset: null, roots: [root] }, "ogf");

      runInAction(() => {
        this.visuals = this.visuals.asReady(visuals);
      });

      this.log.info(`Listed ${visuals.length} visuals in:`, root);
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to list visuals:", transformed);

      runInAction(() => {
        this.visuals = this.visuals.asFailed(transformed, []);
      });
    }
  }

  /** Stop browsing, leaving whatever model is open on screen. */
  @BoundAction()
  public close(): void {
    runInAction(() => {
      this.root = null;
      this.visuals = createLoadable([]);
    });

    setLocalStorageValue(ROOT_STORAGE_KEY, null);
  }
}
