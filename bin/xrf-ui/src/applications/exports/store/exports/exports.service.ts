import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { EApplicationId } from "@/core/router/application";
import { Nullable } from "@/core/types/general";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { emitNotification, ENotificationSeverity } from "@/lib/notifications";
import { commands as exportsEditorCommands } from "@/lib/xrf/bindings/xrf-app-exports-editor";
import { ExportSourceContent, ExportsProject } from "@/lib/xrf/bindings/xrf-export";
import { transformError } from "@/lib/xrf/error";
import { releaseEditorProject } from "@/lib/xrf/ipc";

@Injectable()
export class ExportsService {
  public readonly log: Logger = new Logger(this.constructor.name);

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Nullable<ExportsProject>> = createLoadable(null);

  public constructor(private readonly eventBus: EventBus = inject(EventBus)) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    try {
      const project: Nullable<ExportsProject> = await exportsEditorCommands.getXrExports();

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
        source: EApplicationId.EXPORTS,
        title: "Could not restore the open exports project",
      });
    }
  }

  /** Release parsed exports when the editor is navigated away from. */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(exportsEditorCommands.closeXrExports);
  }

  /**
   * Read back the source declaring one extern.
   *
   * @param name - Name of the declaration to read, as the project reported it.
   * @returns The source text declaring it.
   */
  @BoundAction()
  public async readExportSource(name: string): Promise<ExportSourceContent> {
    this.log.info("Reading export source:", name);

    return exportsEditorCommands.getXrExportSource(name);
  }

  @BoundAction()
  public async openExportsProject(path: string): Promise<void> {
    if (this.project.isLoading) {
      return this.log.info("Skip parsing exports while another operation is running:", path);
    }

    this.log.info("Parsing exports from project:", path);
    this.project = this.project.asLoading(null);

    try {
      const result: ExportsProject = await exportsEditorCommands.openXrExports(path);

      runInAction(() => (this.project = this.project.asReady(result)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to parse exports:", transformed);
      runInAction(() => (this.project = this.project.asFailed(transformed, null)));

      emitNotification(this.eventBus, {
        details: `${path}\n${transformed.message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.EXPORTS,
        title: "Could not parse exports",
      });
    }
  }

  @BoundAction()
  public async refreshExportsProject(): Promise<void> {
    const existing: Nullable<ExportsProject> = this.project.value;

    if (!existing || this.project.isLoading) {
      return;
    }

    this.log.info("Refreshing exports project:", existing.root);
    this.project = this.project.asLoading(existing);

    try {
      const result: ExportsProject = await exportsEditorCommands.openXrExports(existing.root);

      runInAction(() => (this.project = this.project.asReady(result)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to refresh exports project:", transformed);
      runInAction(() => (this.project = this.project.asFailed(transformed, existing)));

      emitNotification(this.eventBus, {
        details: `${existing.root}\n${transformed.message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.EXPORTS,
        title: "Could not refresh exports",
      });
    }
  }

  @BoundAction()
  public async closeExportsProject(): Promise<void> {
    const existing: Nullable<ExportsProject> = this.project.value;

    if (this.project.isLoading) {
      return;
    }

    this.log.info("Closing exports project");
    this.project = this.project.asLoading(existing);

    try {
      await exportsEditorCommands.closeXrExports();
      // Cleared on purpose: closing swaps the viewer for the application's picker in place. It used to
      // hold the project until the caller navigated away, because clearing it unmounted the editor
      // before React Router could process that navigation. Nothing navigates on close any more.
      runInAction(() => (this.project = this.project.asReady(null)));
    } catch (error: unknown) {
      const transformed: Error = transformError(error);

      this.log.error("Failed to close exports project:", transformed);
      runInAction(() => (this.project = this.project.asReady(existing)));

      emitNotification(this.eventBus, {
        details: transformed.message,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.EXPORTS,
        title: "Could not close exports project",
      });

      throw transformed;
    }
  }
}
