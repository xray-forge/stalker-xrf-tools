import { invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { ECommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface ISpawnFileContext {
  spawnActions: {
    openSpawnFile: (path: string) => Promise<void>;
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
}
