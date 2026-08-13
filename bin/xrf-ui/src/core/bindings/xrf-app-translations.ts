// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:translations|close_project"),
  getProject: () =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } } | null>(
      "plugin:translations|get_project"
    ),
  openProject: (path: string) =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } }>(
      "plugin:translations|open_project",
      { path }
    ),
  readProject: (path: string) =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } }>(
      "plugin:translations|read_project",
      { path }
    ),
};

/* Types */
export type TranslationVariant = string | Array<string>;
