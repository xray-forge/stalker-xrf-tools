import { invoke } from "@tauri-apps/api/core";
import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { EApplicationToolId } from "@/core/components/shell/application-tools";
import { Nullable } from "@/core/types/general";
import { transformError } from "@/lib/error";
import { ESpawnsEditorCommand, releaseEditorProject } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { emitNotification, ENotificationSeverity } from "@/lib/notifications";
import { ISpawnFile } from "@/lib/spawn-file";

@Injectable()
export class SpawnFileService {
  @Observable()
  public isReady: boolean = false;

  @Observable()
  public spawnFile: Loadable<Nullable<ISpawnFile>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor(private readonly eventBus: EventBus = inject(EventBus)) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const existing: ISpawnFile = await invoke(ESpawnsEditorCommand.GET_SPAWN_FILE);

    if (existing) {
      this.log.info("Existing spawn file detected");

      runInAction(() => {
        this.spawnFile = createLoadable(existing);
        this.isReady = true;
      });
    } else {
      this.log.info("No existing spawn file");

      runInAction(() => {
        this.isReady = true;
      });
    }
  }

  /**
   * Release the spawn file when the editor is navigated away from.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(ESpawnsEditorCommand.CLOSE_SPAWN_FILE);
  }

  @BoundAction()
  public resetSpawnFile(): void {
    this.spawnFile = createLoadable(null);
  }

  @BoundAction()
  public async openSpawnFile(path: string): Promise<void> {
    this.log.info("Opening spawn file:", path);

    try {
      this.spawnFile = createLoadable(null, true);

      const response: ISpawnFile = await invoke(ESpawnsEditorCommand.OPEN_SPAWN_FILE, { path });

      this.log.info("Spawn file opened");

      runInAction(() => (this.spawnFile = createLoadable(response, false)));
    } catch (error) {
      this.log.error("Failed to open spawn file:", error);

      runInAction(() => (this.spawnFile = createLoadable(null, false, error as Error)));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.SPAWNS,
        title: "Could not open spawn file",
      });
    }
  }

  @BoundAction()
  public async importSpawnFile(path: string): Promise<void> {
    this.log.info("Importing spawn file:", path);

    try {
      this.spawnFile = createLoadable(null, true);

      const response: ISpawnFile = await invoke(ESpawnsEditorCommand.IMPORT_SPAWN_FILE, { path });

      this.log.info("Spawn file imported");

      runInAction(() => (this.spawnFile = createLoadable(response, false)));
    } catch (error) {
      this.log.error("Failed to import spawn file:", error);
      runInAction(() => (this.spawnFile = this.spawnFile.asReady()));

      // The state lands back on ready with no error attached, so without this the failed import is
      // indistinguishable from one that worked.
      emitNotification(this.eventBus, {
        details: `${path}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.SPAWNS,
        title: "Could not import spawn file",
      });
    }
  }

  @BoundAction()
  public async exportSpawnFile(path: string): Promise<void> {
    this.log.info("Exporting spawn file:", path);

    this.assertSpawnFileIsOpen();

    try {
      this.spawnFile = this.spawnFile.asLoading();
      await invoke(ESpawnsEditorCommand.EXPORT_SPAWN_FILE, { path });

      runInAction(() => (this.spawnFile = this.spawnFile.asReady()));

      emitNotification(this.eventBus, {
        details: path,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationToolId.SPAWNS,
        title: "Exported spawn file",
      });
    } catch (error) {
      this.log.error("Failed to export spawn file:", error);

      runInAction(() => (this.spawnFile = this.spawnFile.asReady()));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.SPAWNS,
        title: "Could not export spawn file",
      });
    }
  }

  @BoundAction()
  public async saveSpawnFile(path: string): Promise<void> {
    this.log.info("Saving spawn file:", path);

    this.assertSpawnFileIsOpen();

    try {
      this.spawnFile = this.spawnFile.asLoading();
      await invoke(ESpawnsEditorCommand.SAVE_SPAWN_FILE, { path });

      runInAction(() => (this.spawnFile = this.spawnFile.asReady()));

      emitNotification(this.eventBus, {
        details: path,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationToolId.SPAWNS,
        title: "Saved spawn file",
      });
    } catch (error) {
      this.log.error("Failed to save spawn file:", error);

      runInAction(() => (this.spawnFile = this.spawnFile.asReady()));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.SPAWNS,
        title: "Could not save spawn file",
      });
    }
  }

  @BoundAction()
  public async closeSpawnFile(): Promise<void> {
    this.log.info("Closing existing spawn file");

    try {
      await invoke(ESpawnsEditorCommand.CLOSE_SPAWN_FILE);

      runInAction(() => (this.spawnFile = createLoadable(null)));
    } catch (error) {
      this.log.error("Failed to close spawn file:", error);

      emitNotification(this.eventBus, {
        details: transformError(error).message,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.SPAWNS,
        title: "Could not close spawn file",
      });
    }
  }

  public assertSpawnFileIsOpen(): asserts this is { spawnFile: { value: unknown } } {
    if (this.spawnFile.value === null) {
      throw new Error("Unexpected operation, spawn file is null.");
    }
  }
}
