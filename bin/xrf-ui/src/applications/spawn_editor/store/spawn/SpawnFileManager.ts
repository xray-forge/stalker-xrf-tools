import { invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { ECommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface ISpawnFileContext {
  spawnActions: {
    openSpawnFile: (path: string) => Promise<void>;
    saveSpawnFile: (path: string) => Promise<void>;
    exportSpawnFile: (path: string) => Promise<void>;
    closeSpawnFile: () => Promise<void>;
    resetSpawnFile: () => void;
  };
  isReady: boolean;
  spawnFile: Loadable<Optional<unknown>>;
}

export class SpawnFileManager extends ContextManager<ISpawnFileContext> {
  public context: ISpawnFileContext = {
    spawnActions: createActions({
      openSpawnFile: (path) => this.openSpawnFile(path),
      saveSpawnFile: (path) => this.saveSpawnFile(path),
      exportSpawnFile: (path) => this.exportSpawnFile(path),
      closeSpawnFile: () => this.closeSpawnFile(),
      resetSpawnFile: () => this.setContext({ spawnFile: createLoadable(null) }),
    }),
    isReady: false,
    spawnFile: createLoadable(null),
  };

  public log: Logger = new Logger("spawn");

  public async onProvisionStarted(): Promise<void> {
    const existing: unknown = await invoke(ECommand.GET_SPAWN_FILE);

    if (existing) {
      this.log.info("Existing spawn file detected:", existing);
      this.setContext({ spawnFile: createLoadable(existing), isReady: true });
    } else {
      this.log.info("No existing spawn files:", existing);
      this.setContext({ isReady: true });
    }
  }

  public async openSpawnFile(path: string): Promise<void> {
    this.log.info("Opening spawn file:", path);

    this.setContext({ spawnFile: createLoadable(null, true) });

    try {
      const response: unknown = await invoke(ECommand.OPEN_SPAWN_FILE, { path });

      this.log.info("Spawn file opened");

      this.setContext({ spawnFile: createLoadable(response, false) });
    } catch (error) {
      this.log.error("Failed to open spawn file:", error);
      this.setContext({ spawnFile: createLoadable(null, false, error as Error) });
    }
  }

  public async closeSpawnFile(): Promise<void> {
    this.log.info("Closing existing spawn file");

    try {
      await invoke(ECommand.CLOSE_SPAWN_FILE);
      this.setContext({ spawnFile: createLoadable(null) });
    } catch (error) {
      this.log.error("Failed to close spawn file:", error);
    }
  }

  public async exportSpawnFile(path: string): Promise<void> {
    this.log.info("Exporting spawn file:", path);

    this.assertSpawnFileIsOpen();

    try {
      this.setContext({ spawnFile: this.context.spawnFile.asLoading() });
      await invoke(ECommand.EXPORT_SPAWN_FILE, { path });
      this.setContext({ spawnFile: this.context.spawnFile.asReady() });
    } catch (error) {
      this.log.error("Failed to close spawn file:", error);
      this.setContext({ spawnFile: this.context.spawnFile.asReady() });
    }
  }

  public async saveSpawnFile(path: string): Promise<void> {
    this.log.info("Saving spawn file:", path);

    this.assertSpawnFileIsOpen();

    try {
      this.setContext({ spawnFile: this.context.spawnFile.asLoading() });
      await invoke(ECommand.SAVE_SPAWN_FILE, { path });
      this.setContext({ spawnFile: this.context.spawnFile.asReady() });
    } catch (error) {
      this.log.error("Failed to close spawn file:", error);
      this.setContext({ spawnFile: this.context.spawnFile.asReady() });
    }
  }

  public assertSpawnFileIsOpen(): asserts this is { context: { spawnFile: { value: unknown } } } {
    if (this.context.spawnFile.value === null) {
      throw new Error("Unexpected operation, spawn file is null.");
    }
  }
}
