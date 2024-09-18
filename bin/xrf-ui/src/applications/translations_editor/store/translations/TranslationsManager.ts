import { invoke } from "@tauri-apps/api/core";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { ETranslationsEditorCommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";
import { ITranslationsProjectJson } from "@/lib/translations";

export interface ITranslationsContext {
  translationsActions: {
    open(translationsPath: string): Promise<void>;
    close(): Promise<void>;
  };
  isReady: boolean;
  project: Loadable<Optional<ITranslationsProjectJson>>;
}

export class TranslationsManager extends ContextManager<ITranslationsContext> {
  public context: ITranslationsContext = {
    translationsActions: createActions({
      open: (translationsPath: string) => this.openTranslationsProject(translationsPath),
      close: () => this.closeTranslationsProject(),
    }),
    isReady: false,
    project: createLoadable(null),
  };

  public log: Logger = new Logger("translations");

  public async onProvisionStarted(): Promise<void> {
    const response: ITranslationsProjectJson = await invoke(ETranslationsEditorCommand.GET_TRANSLATIONS_PROJECT);

    if (response) {
      this.log.info("Existing translations project detected");

      this.setContext({
        isReady: true,
        project: createLoadable(response),
      });
    } else {
      this.log.info("No existing translations project");
      this.setContext({ isReady: true });
    }
  }

  public async openTranslationsProject(translationsPath: string): Promise<void> {
    this.log.info("Opening translations project:", translationsPath);

    try {
      this.setContext({ project: createLoadable(null, true) });

      const response: ITranslationsProjectJson = await invoke(ETranslationsEditorCommand.OPEN_TRANSLATIONS_PROJECT, {
        path: translationsPath,
      });

      this.log.info("Translations project opened:", response);

      this.setContext({
        project: createLoadable(response),
      });
    } catch (error) {
      this.log.error("Failed to open translations project:", error);
      this.setContext({ project: createLoadable(null, false, error as Error) });
    }
  }

  public async closeTranslationsProject(): Promise<void> {
    this.log.info("Closing translations project");

    this.setContext(({ project }) => ({ project: project.asLoading() }));

    await invoke(ETranslationsEditorCommand.CLOSE_TRANSLATIONS_PROJECT);

    this.setContext({ project: createLoadable(null) });

    this.log.info("Translations project closed");
  }
}
