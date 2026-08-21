// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import {
  TranslationEdit,
  TranslationFile,
  TranslationFinding,
  TranslationProjectDescriptor,
  TranslationProjectMode,
} from "@/core/bindings/types/xrf-translation";

/** Commands */
export const translationsCommands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:translations|close_project"),
  /** Report which layout a directory looks like, for the open form to preselect. */
  detectMode: (path: string) => __TAURI_INVOKE<TranslationProjectMode>("plugin:translations|detect_mode", { path }),
  getProject: () =>
    __TAURI_INVOKE<{
      mode: TranslationProjectMode;
      root: string;
      /** Every language the root offers, in discovery order. */
      languages: Array<string>;
      /**
       * The code page each language is written in, which is what limits the characters it can hold.
       *
       * Taken from the files themselves in gamedata mode, so a language XRF has never heard of still
       * reports the encoding its own declaration claims.
       */
      encodings: { [key in string]: string };
      files: { [key in string]: TranslationFile };
      findings: Array<TranslationFinding>;
    } | null>("plugin:translations|get_project"),
  openProject: (path: string, mode: TranslationProjectMode) =>
    __TAURI_INVOKE<TranslationProjectDescriptor>("plugin:translations|open_project", { path, mode }),
  /**
   * Write one logical file's pending edits, grouped by the language each belongs to.
   *
   * A logical file is several files on disk in gamedata mode, one per language, so the edits arrive
   * keyed by language and each group goes to its own path. The paths come from the open project rather
   * than from the caller, so a save can only ever touch files this project actually read.
   */
  saveFile: (file: string, edits: { [key in string]: Array<TranslationEdit> }) =>
    __TAURI_INVOKE<TranslationProjectDescriptor>("plugin:translations|save_file", { file, edits }),
  /**
   * Report the first character a language cannot hold, or nothing when the value is writable.
   *
   * Checked here rather than in the interface because the answer depends on code page tables the
   * browser has no encoder for, and on what each language's own files declared. Called when a cell is
   * committed, so a mistake is reported where it was made instead of at the end of a batch save.
   */
  validateText: (language: string, text: string) =>
    __TAURI_INVOKE<string | null>("plugin:translations|validate_text", { language, text }),
};
