import { invoke } from "@tauri-apps/api/core";
import { Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { Nullable } from "@/core/types/general";
import {
  getArchivePreviewSupport,
  IArchiveFileDescriptor,
  IArchiveFileReadResult,
  IArchivesProject,
} from "@/lib/archive";
import { transformError } from "@/lib/error";
import { EArchivesEditorCommand, releaseEditorProject } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";

@Injectable()
export class ArchivesService {
  public readonly log: Logger = new Logger(this.constructor.name);

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Nullable<IArchivesProject>> = createLoadable(null);

  @Observable()
  public file: Loadable<Nullable<IArchiveFileReadResult>> = createLoadable(null);

  @Observable()
  public fileDescriptor: Nullable<IArchiveFileDescriptor> = null;

  private fileRequestId: number = 0;

  public constructor() {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const existing: Nullable<IArchivesProject> = await invoke(EArchivesEditorCommand.GET_ARCHIVES_PROJECT);

    if (existing) {
      this.log.info("Existing archives project detected");

      runInAction(() => {
        this.project = createLoadable(existing);
        this.isReady = true;
      });
    } else {
      this.log.info("No existing archives project");

      runInAction(() => {
        this.isReady = true;
      });
    }
  }

  /**
   * Release the archive project when the editor is navigated away from.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT);
  }

  @BoundAction()
  public resetArchivesProject(): void {
    this.clearFileSelection();
    this.project = createLoadable(null);
  }

  @BoundAction()
  public async openArchivesProject(path: string): Promise<void> {
    this.log.info("Opening archives project:", path);

    try {
      this.clearFileSelection();
      this.project = createLoadable(null, true);

      const response: IArchivesProject = await invoke(EArchivesEditorCommand.OPEN_ARCHIVES_PROJECT, { path });

      this.log.info("Archives project opened");

      runInAction(() => (this.project = createLoadable(response, false)));
    } catch (error: unknown) {
      this.log.error("Failed to open archives project:", error);

      runInAction(() => (this.project = createLoadable(null, false, transformError(error))));
    }
  }

  @BoundAction()
  public async closeArchivesProject(): Promise<void> {
    this.log.info("Closing existing archives project");

    try {
      await invoke(EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT);

      runInAction(() => {
        this.clearFileSelection();
        this.project = createLoadable(null);
      });
    } catch (error: unknown) {
      this.log.error("Failed to close archives project:", error);

      throw transformError(error);
    }
  }

  @BoundAction()
  public async selectArchiveFile(descriptor: IArchiveFileDescriptor): Promise<void> {
    this.fileDescriptor = descriptor;
    this.fileRequestId += 1;
    this.file = createLoadable(null);

    if (!this.isPreviewSupported(descriptor)) {
      return;
    }

    await this.readArchiveFile(descriptor);
  }

  @BoundAction()
  public async retrySelectedFile(): Promise<void> {
    const descriptor: Nullable<IArchiveFileDescriptor> = this.fileDescriptor;

    if (!descriptor || !this.isPreviewSupported(descriptor)) {
      return;
    }

    await this.readArchiveFile(descriptor);
  }

  @BoundAction()
  public clearFileSelection(): void {
    this.fileRequestId += 1;
    this.fileDescriptor = null;
    this.file = createLoadable(null);
  }

  private isPreviewSupported(descriptor: IArchiveFileDescriptor): boolean {
    const project: Nullable<IArchivesProject> = this.project.value;

    return Boolean(project && getArchivePreviewSupport(descriptor, project.readPolicy).kind === "supported");
  }

  private async readArchiveFile(descriptor: IArchiveFileDescriptor): Promise<void> {
    const requestId: number = ++this.fileRequestId;

    this.log.info("Opening archive file:", descriptor.name);
    this.file = createLoadable(null, true);

    try {
      const result: IArchiveFileReadResult = await invoke(EArchivesEditorCommand.READ_ARCHIVE_FILE, {
        path: descriptor.name,
      });

      if (requestId !== this.fileRequestId) {
        return;
      }

      this.log.info("Opened file:", descriptor.name);

      runInAction(() => (this.file = createLoadable(result)));
    } catch (error: unknown) {
      if (requestId !== this.fileRequestId) {
        return;
      }

      this.log.error("Failed to open archive file:", descriptor.name, error);

      runInAction(() => (this.file = createLoadable(null, false, transformError(error))));
    }
  }
}
