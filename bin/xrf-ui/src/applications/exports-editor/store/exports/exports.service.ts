import { invoke } from "@tauri-apps/api/core";
import { inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { ProjectService } from "@/core/store/project";
import { Nullable } from "@/core/types/general";
import { TExportsDeclarations } from "@/lib/exports";
import { EExportsEditorCommand, releaseEditorProject } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";

@Injectable()
export class ExportsService {
  public readonly log: Logger = new Logger(this.constructor.name);

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public declarations: Loadable<Nullable<TExportsDeclarations>> = createLoadable(null);

  public constructor(private readonly projectService: ProjectService = inject(ProjectService)) {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const declarations: Nullable<TExportsDeclarations> = await invoke(EExportsEditorCommand.GET_XR_EXPORTS);

    if (declarations) {
      this.log.info("Existing parsed exports detected");

      runInAction(() => {
        this.declarations = createLoadable(declarations);
        this.isReady = true;
      });
    } else {
      const projectPath: Nullable<string> = this.projectService.xrfProjectPath;

      if (projectPath) {
        this.openExports(projectPath).finally(() => {
          runInAction(() => (this.isReady = true));
        });
      } else {
        this.log.info("No existing parsed effects", projectPath);
        runInAction(() => (this.isReady = true));
      }
    }
  }

  /**
   * Release the parsed exports when the editor is navigated away from.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    releaseEditorProject(EExportsEditorCommand.CLOSE_XR_EXPORTS);
  }

  @BoundAction()
  public async openExports(path: string): Promise<void> {
    if (this.declarations.isLoading) {
      return this.log.info("Skip loading parsing on path:", path);
    }

    this.log.info("Parsing on path:", path);

    try {
      this.declarations = this.declarations.asLoading();

      const result: TExportsDeclarations = await invoke(EExportsEditorCommand.OPEN_XR_EXPORTS, {
        projectPath: path,
      });

      runInAction(() => (this.declarations = createLoadable(result)));
    } catch (error) {
      this.log.error("Got error when parsing exports:", error);
      runInAction(() => (this.declarations = createLoadable(null, false, new Error(error as string))));
    }
  }

  @BoundAction()
  public async closeExports(): Promise<void> {
    this.log.info("Closing exports");

    this.declarations = this.declarations.asLoading();

    await invoke(EExportsEditorCommand.CLOSE_XR_EXPORTS);

    runInAction(() => (this.declarations = createLoadable(null)));
  }
}
