// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeTranslationsProject: () => __TAURI_INVOKE<null>("plugin:translations-editor|close_translations_project"),
  getTranslationsProject: () =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } } | null>(
      "plugin:translations-editor|get_translations_project"
    ),
  openTranslationsProject: (path: string) =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } }>(
      "plugin:translations-editor|open_translations_project",
      { path }
    ),
  readTranslationsProject: (path: string) =>
    __TAURI_INVOKE<{ [key in string]: { [key in string]: { [key in string]: TranslationVariant | null } } }>(
      "plugin:translations-editor|read_translations_project",
      { path }
    ),
};

/* Types */
export type TranslationVariant = string | Array<string>;
