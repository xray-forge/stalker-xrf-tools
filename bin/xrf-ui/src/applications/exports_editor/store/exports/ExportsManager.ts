import { invoke } from "@tauri-apps/api/core";
import { Inject, Injectable, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable } from "@wirestate/react-mobx";

import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { IExportsDeclarations } from "@/lib/exports";
import { EExportsEditorCommand } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";
import {
  getProjectExportConditionsPath,
  getProjectExportDialogsPath,
  getProjectExportEffectsPath,
} from "@/lib/xrf_path";

@Injectable()
export class ExportsManager {
  @Observable()
  public isReady: boolean = false;

  @Observable()
  public declarations: Loadable<Optional<IExportsDeclarations>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor(
    @Inject(ProjectManager)
    private readonly projectManager: ProjectManager
  ) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const declarations: Optional<IExportsDeclarations> = await invoke(EExportsEditorCommand.GET_XR_EXPORTS);

    if (declarations) {
      this.log.info("Existing parsed exports detected");
      this.declarations = createLoadable(declarations);
      this.isReady = true;
    } else {
      const projectPath: Optional<string> = this.projectManager.xrfProjectPath;

      if (projectPath) {
        this.openExports(projectPath).finally(() => {
          this.isReady = true;
        });
      } else {
        this.log.info("No existing parsed effects", projectPath);
        this.isReady = true;
      }
    }
  }

  @BoundAction()
  public async openExports(path: string): Promise<void> {
    if (this.declarations.isLoading) {
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
      this.declarations = this.declarations.asLoading();

      const result: IExportsDeclarations = await invoke(EExportsEditorCommand.OPEN_XR_EXPORTS, {
        effectsPath,
        conditionsPath,
        dialogsPath,
      });

      this.declarations = createLoadable(result);
    } catch (error) {
      this.log.error("Got error when parsing exports:", error);
      this.declarations = createLoadable(null, false, new Error(error as string));
    }
  }

  @BoundAction()
  public async closeExports(): Promise<void> {
    this.log.info("Closing exports");

    this.declarations = this.declarations.asLoading();

    await invoke(EExportsEditorCommand.CLOSE_XR_EXPORTS);

    this.declarations = createLoadable(null);
  }
}
