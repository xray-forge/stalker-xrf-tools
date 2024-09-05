import { exists } from "@tauri-apps/plugin-fs";
import { ContextManager, createActions, OnQuery } from "dreamstate";

import { EProjectQuery } from "@/core/store/project/queries";
import { Optional } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local_storage";
import { Logger } from "@/lib/logging";

export interface IProjectContext {
  projectActions: {
    setXrfProjectPath(path: Optional<string>): void;
    setXrfConfigsPath(path: Optional<string>): void;
  };
  xrfProjectPath: Optional<string>;
  xrfConfigsPath: Optional<string>;
}

export class ProjectManager extends ContextManager<IProjectContext> {
  public log: Logger = new Logger("project");

  public context: IProjectContext = {
    projectActions: createActions({
      setXrfProjectPath: (path) => this.setXrfProjectPath(path),
      setXrfConfigsPath: (path) => this.setXrfConfigsPath(path),
    }),
    xrfProjectPath: null,
    xrfConfigsPath: null,
  };

  public onProvisionStarted(): void {
    this.getXrfProjectPath().then((path) => this.setContext({ xrfProjectPath: path }));
    this.getXrfConfigsPath().then((path) => this.setContext({ xrfConfigsPath: path }));
  }

  public setXrfProjectPath(path: Optional<string>): void {
    this.log.info("Set xrf project path:", path);

    this.setContext({ xrfProjectPath: path });
    setLocalStorageValue("xrf_project_path", path);
  }

  public setXrfConfigsPath(path: Optional<string>): void {
    this.log.info("Set xrf configs path:", path);

    this.setContext({ xrfConfigsPath: path });
    setLocalStorageValue("xrf_configs_path", path);
  }

  public async getXrfProjectPath(): Promise<Optional<string>> {
    const xrfProjectPath: Optional<string> = getLocalStorageValue("xrf_project_path");

    if (xrfProjectPath && (await exists(xrfProjectPath))) {
      this.log.info("Loading xrf project path:", xrfProjectPath);

      return xrfProjectPath;
    }

    return null;
  }

  public async getXrfConfigsPath(): Promise<Optional<string>> {
    const xrfProjectPath: Optional<string> = getLocalStorageValue("xrf_configs_path");

    if (xrfProjectPath && (await exists(xrfProjectPath))) {
      this.log.info("Loading xrf configs path:", xrfProjectPath);

      return xrfProjectPath;
    }

    return null;
  }

  @OnQuery(EProjectQuery.PROJECT_PATH)
  public onGetProjectPath(): Optional<string> {
    return this.context.xrfProjectPath;
  }
}
