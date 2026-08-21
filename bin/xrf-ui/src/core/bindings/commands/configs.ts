// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import { LtxProjectFormatResult, LtxProjectVerifyResult } from "@/core/bindings/types/xrf-ltx";

/** Commands */
export const configsCommands = {
  checkDirectoryFormat: (path: string) =>
    __TAURI_INVOKE<LtxProjectFormatResult>("plugin:configs|check_directory_format", { path }),
  formatDirectory: (path: string) =>
    __TAURI_INVOKE<LtxProjectFormatResult>("plugin:configs|format_directory", { path }),
  verifyDirectory: (path: string) =>
    __TAURI_INVOKE<LtxProjectVerifyResult>("plugin:configs|verify_directory", { path }),
};
