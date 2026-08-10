import { exists } from "@tauri-apps/plugin-fs";
import { Injectable, OnProvision, ProvisionId, WireStatus } from "@wirestate/core";
import { BoundAction, makeObservable, Observable, runInAction } from "@wirestate/mobx";

import { Nullable } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { Logger } from "@/lib/logging";

@Injectable()
export class ProjectService {
  public readonly log: Logger = new Logger(this.constructor.name);

  public readonly status: WireStatus = WireStatus.for(this, { initialize: true });

  @Observable()
  public xrfProjectPath: Nullable<string> = null;

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
  }

  @BoundAction()
  public setXrfProjectPath(path: Nullable<string>): void {
    this.log.info("Set xrf project path:", path);

    this.xrfProjectPath = path;
    setLocalStorageValue("xrf-project-path", path);
  }

  public async getXrfProjectPath(): Promise<Nullable<string>> {
    const xrfProjectPath: Nullable<string> = getLocalStorageValue("xrf-project-path");

    if (xrfProjectPath && (await exists(xrfProjectPath))) {
      return xrfProjectPath;
    }

    return null;
  }
}
