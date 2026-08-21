// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import { ExportDescriptor, ExportSourceContent, ExportsProject } from "@/core/bindings/types/xrf-export";

/** Commands */
export const exportsCommands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:exports|close_project"),
  openProject: (projectPath: string) => __TAURI_INVOKE<ExportsProject>("plugin:exports|open_project", { projectPath }),
  getProject: () =>
    __TAURI_INVOKE<{
      root: string;
      declarations: Array<ExportDescriptor>;
    } | null>("plugin:exports|get_project"),
  getSource: (name: string) => __TAURI_INVOKE<ExportSourceContent>("plugin:exports|get_source", { name }),
};
