import { invoke } from "@tauri-apps/api/core";
import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { EApplicationId } from "@/core/router/application";
import { Nullable } from "@/core/types/general";
import { transformError } from "@/lib/error";
import { ETranslationsEditorCommand, releaseEditorProject } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { emitNotification, ENotificationSeverity } from "@/lib/notifications";
import { ITranslationsProjectJson } from "@/lib/translations";

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
    const response: ITranslationsProjectJson = await invoke(ETranslationsEditorCommand.GET_TRANSLATIONS_PROJECT);

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
   * Release the translations project when the editor is navigated away from.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(ETranslationsEditorCommand.CLOSE_TRANSLATIONS_PROJECT);
  }

  @BoundAction()
  public async openTranslationsProject(translationsPath: string): Promise<void> {
    this.log.info("Opening translations project:", translationsPath);

    try {
      this.project = createLoadable(null, true);

      const response: ITranslationsProjectJson = await invoke(ETranslationsEditorCommand.OPEN_TRANSLATIONS_PROJECT, {
        path: translationsPath,
      });

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
  public async closeTranslationsProject(): Promise<void> {
    this.log.info("Closing translations project");

    this.project = this.project.asLoading();

    await invoke(ETranslationsEditorCommand.CLOSE_TRANSLATIONS_PROJECT);

    runInAction(() => (this.project = createLoadable(null)));

    this.log.info("Translations project closed");
  }
}
