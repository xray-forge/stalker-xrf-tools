import { invoke } from "@tauri-apps/api/core";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { IArchiveFileReadResult, IArchivesProject } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface IArchivesContext {
  archiveActions: {
    open: (path: string) => Promise<void>;
    close: () => Promise<void>;
    reset: () => void;
  };
  fileActions: {
    open: (path: string) => Promise<void>;
  };
  isReady: boolean;
  project: Loadable<Optional<IArchivesProject>>;
  file: Loadable<Optional<IArchiveFileReadResult>>;
}

export class ArchivesManager extends ContextManager<IArchivesContext> {
  public context: IArchivesContext = {
    archiveActions: createActions({
      open: (path) => this.openArchivesProject(path),
      close: () => this.closeArchivesProject(),
      reset: () => this.setContext({ project: createLoadable(null) }),
    }),
    fileActions: createActions({
      open: (path) => this.openArchiveFile(path),
    }),
    isReady: false,
    project: createLoadable(null),
    file: createLoadable(null),
  };

  public log: Logger = new Logger("archives");

  public async onProvisionStarted(): Promise<void> {
    const existing: IArchivesProject = await invoke(EArchivesEditorCommand.GET_ARCHIVES_PROJECT);

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

      const response: IArchivesProject = await invoke(EArchivesEditorCommand.OPEN_ARCHIVES_PROJECT, { path });

      this.log.info("Archives project opened");

      this.setContext({ project: createLoadable(response, false) });
    } catch (error) {
      this.log.error("Failed to open archives project:", error);
      this.setContext({ project: createLoadable(null, false, error as Error) });
    }
  }

  public async closeArchivesProject(): Promise<void> {
    this.log.info("Closing existing archives project");

    try {
      await invoke(EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT);
      this.setContext({ project: createLoadable(null) });
    } catch (error) {
      this.log.error("Failed to close archives project:", error);
    }
  }

  public async openArchiveFile(path: string): Promise<void> {
    this.log.info("Opening archive file:", path);

    this.setContext(({ file }) => ({
      file: file.asLoading(),
    }));

    try {
      const result: IArchiveFileReadResult = await invoke(EArchivesEditorCommand.READ_ARCHIVE_FILE, { path });

      this.log.info("Opened file:", path);

      this.setContext({ file: createLoadable(result) });
    } catch (error) {
      this.log.error("Failed to open archive file:", path, error);
      this.setContext({ file: createLoadable(null, false, new Error(String(error))) });
    }
  }
}
