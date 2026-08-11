import { invoke } from "@tauri-apps/api/core";
import { Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { Nullable } from "@/core/types/general";
import {
  getArchivePreviewSupport,
  IArchiveFileDescriptor,
  IArchiveFileReadResult,
  IArchiveFolderExtractResult,
  IArchiveImagePreview,
  IArchivesProject,
  isArchiveImage,
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

  /** Decoded texture for the selected file, when it is one the backend can turn into a picture. */
  @Observable()
  public image: Loadable<Nullable<IArchiveImagePreview>> = createLoadable(null);

  /** Outcome of the last single file extraction, holding the path it was written to when it worked. */
  @Observable()
  public singleFileExtraction: Loadable<Nullable<string>> = createLoadable(null);

  /**
   * Archive-relative path of the selected directory, empty string for the archive root.
   *
   * Held separately from `fileDescriptor` rather than as one selection union: only one of the two can
   * be set, and keeping them apart means neither view has to interrogate the other's shape.
   */
  @Observable()
  public directoryPath: Nullable<string> = null;

  /** Outcome of the last folder extraction, holding how much was written when it worked. */
  @Observable()
  public folderExtraction: Loadable<Nullable<IArchiveFolderExtractResult>> = createLoadable(null);

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
    this.directoryPath = null;
    this.fileRequestId += 1;
    this.file = createLoadable(null);
    this.image = createLoadable(null);

    if (this.isImage(descriptor)) {
      await this.readArchiveImage(descriptor);

      return;
    }

    if (!this.isPreviewSupported(descriptor)) {
      return;
    }

    await this.readArchiveFile(descriptor);
  }

  @BoundAction()
  public async retrySelectedFile(): Promise<void> {
    const descriptor: Nullable<IArchiveFileDescriptor> = this.fileDescriptor;

    if (!descriptor) {
      return;
    }

    // Retry has to repeat whichever read failed. Always re-reading as text would leave a failed image
    // showing a decode error that the retry button could never clear.
    if (this.isImage(descriptor)) {
      await this.readArchiveImage(descriptor);

      return;
    }

    if (!this.isPreviewSupported(descriptor)) {
      return;
    }

    await this.readArchiveFile(descriptor);
  }

  /**
   * Write one archived file to a path of the user's choosing.
   */
  @BoundAction()
  public async extractArchiveFile(descriptor: IArchiveFileDescriptor, destination: string): Promise<void> {
    this.log.info("Extracting archive file:", descriptor.name, destination);

    try {
      this.singleFileExtraction = createLoadable(null, true);

      await invoke(EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE, { name: descriptor.name, destination });

      runInAction(() => (this.singleFileExtraction = createLoadable(destination)));
    } catch (error) {
      this.log.error("Failed to extract archive file:", error);

      runInAction(() => (this.singleFileExtraction = createLoadable(null, false, error as Error)));

      throw error;
    }
  }

  /** Dismiss whatever the last extraction reported, success or failure. */
  @BoundAction()
  public clearExtraction(): void {
    this.singleFileExtraction = createLoadable(null);
  }

  /** Select a directory, which is a different kind of selection than a file rather than a wider one. */
  @BoundAction()
  public selectArchiveDirectory(path: string): void {
    this.fileRequestId += 1;
    this.fileDescriptor = null;
    this.file = createLoadable(null);
    this.image = createLoadable(null);
    this.directoryPath = path;
    this.folderExtraction = createLoadable(null);
  }

  /**
   * Write every archived file under a directory into a destination root.
   *
   * An empty prefix extracts the whole archive, which is what selecting the tree root means.
   */
  @BoundAction()
  public async extractArchiveFolder(prefix: string, destination: string): Promise<void> {
    this.log.info("Extracting archive folder:", prefix || "<root>", destination);

    try {
      this.folderExtraction = createLoadable(null, true);

      const result: IArchiveFolderExtractResult = await invoke(EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER, {
        prefix,
        destination,
      });

      runInAction(() => (this.folderExtraction = createLoadable(result)));
    } catch (error: unknown) {
      this.log.error("Failed to extract archive folder:", error);

      runInAction(() => (this.folderExtraction = createLoadable(null, false, transformError(error))));

      throw transformError(error);
    }
  }

  /** Dismiss whatever the last folder extraction reported. */
  @BoundAction()
  public clearFolderExtraction(): void {
    this.folderExtraction = createLoadable(null);
  }

  @BoundAction()
  public clearFileSelection(): void {
    this.fileRequestId += 1;
    this.fileDescriptor = null;
    this.directoryPath = null;
    this.file = createLoadable(null);
    this.image = createLoadable(null);
    this.folderExtraction = createLoadable(null);
  }

  private isImage(descriptor: IArchiveFileDescriptor): boolean {
    const project: Nullable<IArchivesProject> = this.project.value;

    return Boolean(project && isArchiveImage(descriptor, project.readPolicy));
  }

  private isPreviewSupported(descriptor: IArchiveFileDescriptor): boolean {
    const project: Nullable<IArchivesProject> = this.project.value;

    return Boolean(project && getArchivePreviewSupport(descriptor, project.readPolicy).kind === "supported");
  }

  /**
   * Ask the backend to decode an archived texture into something the webview can show.
   *
   * Guarded by the same request identifier as text reads: clicking through a directory of textures
   * starts a decode per file, and an earlier one finishing last would otherwise win.
   */
  private async readArchiveImage(descriptor: IArchiveFileDescriptor): Promise<void> {
    const requestId: number = ++this.fileRequestId;

    this.log.info("Reading archive image:", descriptor.name);
    this.image = createLoadable(null, true);

    try {
      const result: IArchiveImagePreview = await invoke(EArchivesEditorCommand.READ_ARCHIVE_IMAGE, {
        path: descriptor.name,
      });

      if (requestId !== this.fileRequestId) {
        return;
      }

      runInAction(() => (this.image = createLoadable(result)));
    } catch (error: unknown) {
      if (requestId !== this.fileRequestId) {
        return;
      }

      this.log.error("Failed to read archive image:", descriptor.name, error);

      runInAction(() => (this.image = createLoadable(null, false, transformError(error))));
    }
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
