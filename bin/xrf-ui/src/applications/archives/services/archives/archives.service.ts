import {
  EventBus,
  inject,
  Injectable,
  OnDeactivation,
  OnDeprovision,
  OnProvision,
  ProvisionId,
  WireStatus,
} from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import {
  getArchivePreviewSupport,
  isArchiveAudio,
  isArchiveImage,
  TArchiveContent,
  TArchiveOperation,
  TArchiveSelection,
} from "@/core/archive";
import { commands as archivesEditorCommands } from "@/core/bindings/xrf-app-archives-editor";
import { ArchiveExtractFolderResult, ArchiveFileDescriptor, ArchiveProject } from "@/core/bindings/xrf-archive";
import { transformError } from "@/core/error/lib";
import { releaseEditorProject } from "@/core/ipc/release";
import { emitNotification, ENotificationSeverity } from "@/core/notifications/lib";
import { EApplicationId } from "@/core/routing/application";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

@Injectable()
export class ArchivesService {
  public readonly log: Logger = new Logger(this.constructor.name);

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Nullable<ArchiveProject>> = createLoadable(null);

  /** What the explorer points at. Exactly one kind at a time, by construction. */
  @Observable()
  public selection: TArchiveSelection = { kind: "none" };

  /** Whatever was loaded for the selection - text, a decoded texture, and later audio. */
  @Observable()
  public content: Loadable<Nullable<TArchiveContent>> = createLoadable(null);

  /** The last write to disk, so whichever surface started it can report the outcome. */
  @Observable()
  public operation: Loadable<Nullable<TArchiveOperation>> = createLoadable(null);

  private contentRequestId: number = 0;

  /** The selected file, or null when a directory or nothing is selected. */
  public get selectedFile(): Nullable<ArchiveFileDescriptor> {
    return this.selection.kind === "file" ? this.selection.descriptor : null;
  }

  /** The selected directory, empty string for the archive root, or null when a file is selected. */
  public get selectedDirectory(): Nullable<string> {
    return this.selection.kind === "directory" ? this.selection.path : null;
  }

  /**
   * Whether something is in flight that a second command would race.
   *
   * Read by the editor to lock navigation and by the explorer to refuse a new selection. Derived here
   * rather than reassembled at each call site, which is how the three copies of it drifted apart.
   */
  public get isBusy(): boolean {
    return this.content.isLoading || this.operation.isLoading;
  }

  public constructor(
    private readonly status: WireStatus = WireStatus.for(this, { initialize: true }),
    private readonly eventBus: EventBus = inject(EventBus)
  ) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(provisionId: ProvisionId): Promise<void> {
    this.log.info("Provisioning:", provisionId);

    const existing: Nullable<ArchiveProject> = await archivesEditorCommands.getArchivesProject();

    if (this.status.provisionId !== provisionId) {
      return this.log.info("Discard outdated get archives request:", provisionId, "<", this.status.provisionId);
    }

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

  @OnDeprovision()
  public onDeprovision(provisionId: ProvisionId): void {
    this.log.info("Deprovisioning:", provisionId);
  }

  /**
   * Release the archive project when the editor is navigated away from.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    this.log.info("Deactivating");

    releaseEditorProject(archivesEditorCommands.closeArchivesProject);
  }

  @BoundAction()
  public resetArchivesProject(): void {
    this.log.info("Reset archives project");

    this.clearFileSelection();
    this.project = createLoadable(null);
  }

  @BoundAction()
  public async openArchivesProject(path: string): Promise<void> {
    this.log.info("Opening archives project:", path);

    try {
      this.clearFileSelection();
      this.project = createLoadable(null, true);

      const response: ArchiveProject = await archivesEditorCommands.openArchivesProject(path);

      this.log.info("Archives project opened");

      runInAction(() => (this.project = createLoadable(response, false)));
    } catch (error: unknown) {
      this.log.error("Failed to open archives project:", error);

      runInAction(() => (this.project = createLoadable(null, false, transformError(error))));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.ARCHIVES,
        title: "Could not open archives project",
      });
    }
  }

  @BoundAction()
  public async closeArchivesProject(): Promise<void> {
    this.log.info("Closing existing archives project");

    try {
      await archivesEditorCommands.closeArchivesProject();

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
  public async selectArchiveFile(descriptor: ArchiveFileDescriptor): Promise<void> {
    this.log.info("Select archive file:", descriptor);

    this.selection = { kind: "file", descriptor };
    this.contentRequestId += 1;
    this.content = createLoadable(null);

    await this.loadSelectedContent(descriptor);
  }

  /** Select a directory, which is a different kind of selection than a file rather than a wider one. */
  @BoundAction()
  public selectArchiveDirectory(path: string): void {
    this.contentRequestId += 1;
    this.selection = { kind: "directory", path };
    this.content = createLoadable(null);
    this.operation = createLoadable(null);
  }

  @BoundAction()
  public async retrySelectedFile(): Promise<void> {
    const descriptor: Nullable<ArchiveFileDescriptor> = this.selectedFile;

    if (descriptor) {
      await this.loadSelectedContent(descriptor);
    }
  }

  /**
   * Write one archived file to a path of the user's choosing.
   */
  @BoundAction()
  public async extractArchiveFile(descriptor: ArchiveFileDescriptor, destination: string): Promise<void> {
    this.log.info("Extracting archive file:", descriptor.name, destination);

    try {
      this.operation = createLoadable(null, true);

      await archivesEditorCommands.extractArchiveFile(descriptor.name, destination);

      runInAction(() => (this.operation = createLoadable({ kind: "extract-file", destination })));

      emitNotification(this.eventBus, {
        details: destination,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.ARCHIVES,
        title: `Extracted ${descriptor.name}`,
      });
    } catch (error: unknown) {
      this.log.error("Failed to extract archive file:", error);

      runInAction(() => (this.operation = createLoadable(null, false, transformError(error))));

      emitNotification(this.eventBus, {
        details: `${destination}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.ARCHIVES,
        title: `Could not extract ${descriptor.name}`,
      });

      throw transformError(error);
    }
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
      this.operation = createLoadable(null, true);

      const result: ArchiveExtractFolderResult = await archivesEditorCommands.extractArchiveFolder(prefix, destination);

      runInAction(() => (this.operation = createLoadable({ kind: "extract-folder", result })));

      // Reported without a count rather than not at all: a response the parser did not fill in is no
      // reason to turn a write that happened into a thrown error.
      const extractedCount: Nullable<number> = result?.extractedCount ?? null;
      const extractedFrom: string = prefix || "the archive root";

      emitNotification(this.eventBus, {
        details: destination,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.ARCHIVES,
        title:
          extractedCount === null
            ? `Extracted ${extractedFrom}`
            : `Extracted ${extractedCount} file(s) from ${extractedFrom}`,
      });
    } catch (error: unknown) {
      this.log.error("Failed to extract archive folder:", error);

      runInAction(() => (this.operation = createLoadable(null, false, transformError(error))));

      emitNotification(this.eventBus, {
        details: `${destination}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.ARCHIVES,
        title: `Could not extract ${prefix || "the archive root"}`,
      });

      throw transformError(error);
    }
  }

  /** Dismiss whatever the last write reported, success or failure. */
  @BoundAction()
  public clearOperation(): void {
    this.operation = createLoadable(null);
  }

  @BoundAction()
  public clearFileSelection(): void {
    this.contentRequestId += 1;
    this.selection = { kind: "none" };
    this.content = createLoadable(null);
    this.operation = createLoadable(null);
  }

  /**
   * Load whatever the selected file can be shown as.
   *
   * The single entry point for every content kind, so selecting and retrying cannot disagree about
   * which one applies - they did, until retry started re-reading textures as text.
   */
  private async loadSelectedContent(descriptor: ArchiveFileDescriptor): Promise<void> {
    const project: Nullable<ArchiveProject> = this.project.value;

    if (!project) {
      return;
    }

    if (isArchiveAudio(descriptor, project.readPolicy)) {
      return await this.readContent(descriptor, "audio");
    }

    if (isArchiveImage(descriptor, project.readPolicy)) {
      return await this.readContent(descriptor, "image");
    }

    if (getArchivePreviewSupport(descriptor, project.readPolicy).kind === "supported") {
      return await this.readContent(descriptor, "text");
    }
  }

  /**
   * Ask the backend for one representation of a file and publish it as the current content.
   *
   * Guarded by a request identifier: clicking through a directory starts a read per file, and an
   * earlier one finishing last would otherwise overwrite a newer selection's content.
   */
  private async readContent(descriptor: ArchiveFileDescriptor, kind: TArchiveContent["kind"]): Promise<void> {
    const requestId: number = ++this.contentRequestId;

    this.log.info("Reading archive content:", kind, descriptor.name);
    this.content = createLoadable(null, true);

    try {
      const content: TArchiveContent =
        kind === "audio"
          ? {
              kind: "audio",
              preview: await archivesEditorCommands.readArchiveAudio(descriptor.name),
            }
          : kind === "image"
            ? {
                kind: "image",
                preview: await archivesEditorCommands.readArchiveImage(descriptor.name),
              }
            : {
                kind: "text",
                result: await archivesEditorCommands.readArchiveFile(descriptor.name),
              };

      if (requestId !== this.contentRequestId) {
        return;
      }

      runInAction(() => (this.content = createLoadable(content)));
    } catch (error: unknown) {
      if (requestId !== this.contentRequestId) {
        return;
      }

      this.log.error("Failed to read archive content:", descriptor.name, error);

      runInAction(() => (this.content = createLoadable(null, false, transformError(error))));
    }
  }
}
