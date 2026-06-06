import { invoke } from "@tauri-apps/api/core";
import { Injectable, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/react-mobx";

import { Optional } from "@/core/types/general";
import { ETranslationsEditorCommand } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import { ITranslationsProjectJson } from "@/lib/translations";

@Injectable()
export class TranslationsManager {
  @Observable()
  public isReady: boolean = false;

  @Observable()
  public project: Loadable<Optional<ITranslationsProjectJson>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor() {
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
