import { invoke } from "@tauri-apps/api/core";
import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { EApplicationToolId } from "@/core/components/shell/application-tools";
import { Nullable } from "@/core/types/general";
import { transformError } from "@/lib/error";
import { IExportSourceContent, IExportsProject } from "@/lib/exports";
import { EExportsEditorCommand, releaseEditorProject } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { emitNotification, ENotificationSeverity } from "@/lib/notifications";

@Injectable()
export class ExportsService {
  public readonly log: Logger = new Logger(this.constructor.name);

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Nullable<IExportsProject>> = createLoadable(null);

  public constructor(private readonly eventBus: EventBus = inject(EventBus)) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    try {
      const project: Nullable<IExportsProject> = await invoke(EExportsEditorCommand.GET_XR_EXPORTS);

      if (project) {
        this.log.info("Existing exports project detected");
      } else {
        this.log.info("No existing exports project");
      }

      runInAction(() => {
        this.project = createLoadable(project);
        this.isReady = true;
      });
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to restore exports project:", transformed);

      runInAction(() => {
        this.project = this.project.asFailed(transformed, null);
        this.isReady = true;
      });

      emitNotification(this.eventBus, {
        details: transformed.message,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.EXPORTS,
        title: "Could not restore the open exports project",
      });
    }
  }

  /** Release parsed exports when the editor is navigated away from. */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(EExportsEditorCommand.CLOSE_XR_EXPORTS);
  }

  /**
   * Read back the source declaring one extern.
   *
   * @param name - Name of the declaration to read, as the project reported it.
   * @returns The source text declaring it.
   */
  @BoundAction()
  public async readExportSource(name: string): Promise<IExportSourceContent> {
    this.log.info("Reading export source:", name);

    return invoke(EExportsEditorCommand.GET_XR_EXPORT_SOURCE, { name });
  }

  @BoundAction()
  public async openExportsProject(path: string): Promise<void> {
    if (this.project.isLoading) {
      return this.log.info("Skip parsing exports while another operation is running:", path);
    }

    this.log.info("Parsing exports from project:", path);
    this.project = this.project.asLoading(null);

    try {
      const result: IExportsProject = await invoke(EExportsEditorCommand.OPEN_XR_EXPORTS, {
        projectPath: path,
      });

      runInAction(() => (this.project = this.project.asReady(result)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to parse exports:", transformed);
      runInAction(() => (this.project = this.project.asFailed(transformed, null)));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformed.message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.EXPORTS,
        title: "Could not parse exports",
      });
    }
  }

  @BoundAction()
  public async refreshExportsProject(): Promise<void> {
    const existing: Nullable<IExportsProject> = this.project.value;

    if (!existing || this.project.isLoading) {
      return;
    }

    this.log.info("Refreshing exports project:", existing.root);
    this.project = this.project.asLoading(existing);

    try {
      const result: IExportsProject = await invoke(EExportsEditorCommand.OPEN_XR_EXPORTS, {
        projectPath: existing.root,
      });

      runInAction(() => (this.project = this.project.asReady(result)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to refresh exports project:", transformed);
      runInAction(() => (this.project = this.project.asFailed(transformed, existing)));

      emitNotification(this.eventBus, {
        details: `${existing.root}\n${transformed.message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.EXPORTS,
        title: "Could not refresh exports",
      });
    }
  }

  @BoundAction()
  public async closeExportsProject(): Promise<void> {
    const existing: Nullable<IExportsProject> = this.project.value;

    if (this.project.isLoading) {
      return;
    }

    this.log.info("Closing exports project");
    this.project = this.project.asLoading(existing);

    try {
      await invoke(EExportsEditorCommand.CLOSE_XR_EXPORTS);
      // Keep the rendered project alive until its caller navigates away; clearing it here unmounts the
      // editor before React Router can process that navigation.
      runInAction(() => (this.project = this.project.asReady(existing)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to close exports project:", transformed);
      runInAction(() => (this.project = this.project.asReady(existing)));

      emitNotification(this.eventBus, {
        details: transformed.message,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.EXPORTS,
        title: "Could not close exports project",
      });

      throw transformed;
    }
  }
}
