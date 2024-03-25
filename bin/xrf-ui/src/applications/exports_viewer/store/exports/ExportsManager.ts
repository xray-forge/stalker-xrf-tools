import { invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { queryProjectPath } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { IExportsDeclarations } from "@/lib/exports";
import { ECommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";
import {
  getProjectExportConditionsPath,
  getProjectExportDialogsPath,
  getProjectExportEffectsPath,
} from "@/lib/xrf_path";

export interface IExportsContext {
  exportsActions: {
    open: (path: string) => Promise<void>;
    close: () => Promise<void>;
  };
  isReady: boolean;
  declarations: Loadable<Optional<IExportsDeclarations>>;
}

export class ExportsManager extends ContextManager<IExportsContext> {
  public context: IExportsContext = {
    exportsActions: createActions({ open: (path) => this.openExports(path), close: () => this.closeExports() }),
    isReady: false,
    declarations: createLoadable(null),
  };

  public log: Logger = new Logger("exports");

  public async onProvisionStarted(): Promise<void> {
    const declarations: Optional<IExportsDeclarations> = await invoke(ECommand.GET_XR_EXPORTS);

    if (declarations) {
      this.log.info("Existing parsed exports detected");
      this.setContext({
        declarations: createLoadable(declarations),
        isReady: true,
      });
    } else {
      const projectPath: Optional<string> = queryProjectPath(this);

      if (projectPath) {
        this.openExports(projectPath).finally(() => {
          this.setContext({ isReady: true });
        });
      } else {
        this.log.info("No existing parsed effects", projectPath);
        this.setContext({ isReady: true });
      }
    }
  }

  public async openExports(path: string): Promise<void> {
    if (this.context.declarations.isLoading) {
      return this.log.info("Skip loading parsing on path:", path);
    }

    this.log.info("Parsing on path:", path);

    const [effectsPath, conditionsPath, dialogsPath] = await Promise.all([
      getProjectExportEffectsPath(path),
      getProjectExportConditionsPath(path),
      getProjectExportDialogsPath(path),
    ]);

    this.log.info("Parsing on paths:", effectsPath, conditionsPath, dialogsPath);

    try {
      this.setContext(({ declarations }) => ({ declarations: declarations.asLoading() }));

      const result: IExportsDeclarations = await invoke(ECommand.OPEN_XR_EXPORTS, {
        effectsPath,
        conditionsPath,
        dialogsPath,
      });

      this.setContext({ declarations: createLoadable(result) });
    } catch (error) {
      this.log.error("Got error when parsing exports:", error);
      this.setContext({ declarations: createLoadable(null, false, new Error(error as string)) });
    }
  }

  public async closeExports(): Promise<void> {
    this.log.info("Closing exports");

    this.setContext(({ declarations }) => ({ declarations: declarations.asLoading() }));

    await invoke(ECommand.CLOSE_XR_EXPORTS);

    this.setContext({ declarations: createLoadable(null) });
  }
}
