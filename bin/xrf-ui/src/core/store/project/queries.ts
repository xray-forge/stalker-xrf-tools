import { ContextManager } from "dreamstate";

import { AnyObject, Optional } from "@/core/types/general";

export enum EProjectQuery {
  PROJECT_PATH = "PROJECT_PATH",
}

export function queryProjectPath<T extends ContextManager<AnyObject>>(manager: T): Optional<string> {
  return manager.IS_DISPOSED ? null : manager.queryDataSync({ type: EProjectQuery.PROJECT_PATH })?.data;
}
