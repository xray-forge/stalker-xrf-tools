import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { commands as translationsCommands } from "@/core/bindings/xrf-app-translations";
import { transformError } from "@/core/error/lib";
import { releaseEditorProject } from "@/core/ipc/release";
import { emitNotification, ENotificationSeverity } from "@/core/notifications/lib";
import { EApplicationId } from "@/core/routing/application";
import { ITranslationsProjectJson } from "@/core/translations";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

@Injectable()
export class TranslationsService {
  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Nullable<ITranslationsProjectJson>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor(private readonly eventBus: EventBus = inject(EventBus)) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const response: Nullable<ITranslationsProjectJson> = await translationsCommands.getProject();

    if (response) {
      this.log.info("Existing translations project detected");

      runInAction(() => {
        this.isReady = true;
        this.project = createLoadable(response);
      });
    } else {
      this.log.info("No existing translations project");

      runInAction(() => {
        this.isReady = true;
      });
    }
  }

  /**
   * Releases the translations project when the editor deactivates.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(translationsCommands.closeProject);
  }

  @BoundAction()
  public async openProject(translationsPath: string): Promise<void> {
    this.log.info("Opening translations project:", translationsPath);

    try {
      this.project = createLoadable(null, true);

      const response: ITranslationsProjectJson = await translationsCommands.openProject(translationsPath);

      this.log.info("Translations project opened:", response);

      runInAction(() => (this.project = createLoadable(response)));
    } catch (error) {
      this.log.error("Failed to open translations project:", error);

      runInAction(() => (this.project = createLoadable(null, false, error as Error)));

      emitNotification(this.eventBus, {
        details: `${translationsPath}\n${transformError(error).message}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.TRANSLATIONS,
        title: "Could not open translations project",
      });
    }
  }

  @BoundAction()
  public async closeProject(): Promise<void> {
    this.log.info("Closing translations project");

    this.project = this.project.asLoading();

    await translationsCommands.closeProject();

    runInAction(() => (this.project = createLoadable(null)));

    this.log.info("Translations project closed");
  }
}
