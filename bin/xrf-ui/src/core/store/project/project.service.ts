import { exists } from "@tauri-apps/plugin-fs";
import { Injectable, OnProvision, ProvisionId, WireStatus } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { Optional } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local_storage";
import { Logger } from "@/lib/logging";

@Injectable()
export class ProjectService {
  public readonly log: Logger = new Logger(this.constructor.name);

  public readonly status: WireStatus = WireStatus.for(this, { initialize: true });

  @Observable()
  public xrfProjectPath: Optional<string> = null;

  @Observable()
  public xrfConfigsPath: Optional<string> = null;

  public constructor() {
    makeObservable(this);
  }

  @OnProvision()
  public onProvision(provisionId: ProvisionId): void {
    this.getXrfProjectPath().then((path) => {
      if (provisionId === this.status.provisionId) {
        this.log.info("Loaded getXrfProjectPath:", path);
        runInAction(() => (this.xrfProjectPath = path));
      }
    });

    this.getXrfConfigsPath().then((path) => {
      if (provisionId === this.status.provisionId) {
        this.log.info("Loaded getXrfConfigsPath:", path);
        runInAction(() => (this.xrfConfigsPath = path));
      }
    });
  }

  @BoundAction()
  public setXrfProjectPath(path: Optional<string>): void {
    this.log.info("Set xrf project path:", path);

    this.xrfProjectPath = path;
    setLocalStorageValue("xrf-project-path", path);
  }

  @BoundAction()
  public setXrfConfigsPath(path: Optional<string>): void {
    this.log.info("Set xrf configs path:", path);

    this.xrfConfigsPath = path;
    setLocalStorageValue("xrf-configs-path", path);
  }

  public async getXrfProjectPath(): Promise<Optional<string>> {
    const xrfProjectPath: Optional<string> = getLocalStorageValue("xrf-project-path");

    if (xrfProjectPath && (await exists(xrfProjectPath))) {
      return xrfProjectPath;
    }

    return null;
  }

  public async getXrfConfigsPath(): Promise<Optional<string>> {
    const xrfProjectPath: Optional<string> = getLocalStorageValue("xrf-configs-path");

    if (xrfProjectPath && (await exists(xrfProjectPath))) {
      return xrfProjectPath;
    }

    return null;
  }
}
