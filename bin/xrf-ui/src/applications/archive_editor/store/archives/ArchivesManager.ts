import { invoke } from "@tauri-apps/api/core";
import { OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable } from "@wirestate/react-mobx";

import { Optional } from "@/core/types/general";
import { IArchiveFileReadResult, IArchivesProject } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";

export class ArchivesManager {
  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Optional<IArchivesProject>> = createLoadable(null);

  @Observable()
  public file: Loadable<Optional<IArchiveFileReadResult>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor() {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const existing: IArchivesProject = await invoke(EArchivesEditorCommand.GET_ARCHIVES_PROJECT);

    if (existing) {
      this.log.info("Existing archives project detected");
      this.project = createLoadable(existing);
      this.isReady = true;
    } else {
      this.log.info("No existing archives project");
      this.isReady = true;
    }
  }

  @BoundAction()
  public resetArchivesProject(): void {
    this.project = createLoadable(null);
  }

  @BoundAction()
  public async openArchivesProject(path: string): Promise<void> {
    this.log.info("Opening archives project:", path);

    try {
      this.project = createLoadable(null, true);

      const response: IArchivesProject = await invoke(EArchivesEditorCommand.OPEN_ARCHIVES_PROJECT, { path });

      this.log.info("Archives project opened");

      this.project = createLoadable(response, false);
    } catch (error) {
      this.log.error("Failed to open archives project:", error);
      this.project = createLoadable(null, false, error as Error);
    }
  }

  @BoundAction()
  public async closeArchivesProject(): Promise<void> {
    this.log.info("Closing existing archives project");

    try {
      await invoke(EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT);
      this.project = createLoadable(null);
    } catch (error) {
      this.log.error("Failed to close archives project:", error);
    }
  }

  @BoundAction()
  public async openArchiveFile(path: string): Promise<void> {
    this.log.info("Opening archive file:", path);

    this.file = this.file.asLoading();

    try {
      const result: IArchiveFileReadResult = await invoke(EArchivesEditorCommand.READ_ARCHIVE_FILE, { path });

      this.log.info("Opened file:", path);

      this.file = createLoadable(result);
    } catch (error) {
      this.log.error("Failed to open archive file:", path, error);
      this.file = createLoadable(null, false, new Error(String(error)));
    }
  }
}
