import { invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { IArchivesProject } from "@/lib/archive";
import { ECommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface IArchivesContext {
  archiveActions: {
    open: (path: string) => Promise<void>;
    close: () => Promise<void>;
    reset: () => void;
  };
  isReady: boolean;
  project: Loadable<Optional<IArchivesProject>>;
}

export class ArchivesManager extends ContextManager<IArchivesContext> {
  public context: IArchivesContext = {
    archiveActions: createActions({
      open: (path) => this.openArchivesProject(path),
      close: () => this.closeArchivesProject(),
      reset: () => this.setContext({ project: createLoadable(null) }),
    }),
    isReady: false,
    project: createLoadable(null),
  };

  public log: Logger = new Logger("archives");

  public async onProvisionStarted(): Promise<void> {
    const existing: IArchivesProject = await invoke(ECommand.GET_ARCHIVES_PROJECT);

    if (existing) {
      this.log.info("Existing archives project detected");
      this.setContext({ project: createLoadable(existing), isReady: true });
    } else {
      this.log.info("No existing archives project");
      this.setContext({ isReady: true });
    }
  }

  public async openArchivesProject(path: string): Promise<void> {
    this.log.info("Opening archives project:", path);

    try {
      this.setContext({ project: createLoadable(null, true) });

      const response: IArchivesProject = await invoke(ECommand.OPEN_ARCHIVES_PROJECT, { path });

      this.log.info("Archives project opened:", Object.keys(response));

      this.setContext({ project: createLoadable(response, false) });
    } catch (error) {
      this.log.error("Failed to open archives project:", error);
      this.setContext({ project: createLoadable(null, false, error as Error) });
    }
  }

  public async closeArchivesProject(): Promise<void> {
    this.log.info("Closing existing archives project");

    try {
      await invoke(ECommand.CLOSE_ARCHIVES_PROJECT);
      this.setContext({ project: createLoadable(null) });
    } catch (error) {
      this.log.error("Failed to close archives project:", error);
    }
  }
}
