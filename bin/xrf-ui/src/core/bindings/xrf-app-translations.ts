// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
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

/* Types */
/**
 * One change to a translation entry, in whichever kind of file holds it.
 *
 * Format-neutral on purpose. It used to live beside the XML writer and carry a bare `String`, which
 * quietly could not express what a JSON source already holds: an entry whose text is an array of
 * lines. Editing one of those flattened it on save, and roughly 190 entries across ten files in the
 * engine's own translations are that shape.
 */
export type TranslationEdit =
  /** Replace the winning entry's value, or append the entry when the file has none. */
  | { kind: "set"; id: string; value: TranslationVariant }
  /** Remove the entry entirely, shadowed duplicates included. */
  | { kind: "remove"; id: string };

/** One logical translation file, and where each language's copy of it lives. */
export type TranslationFile = {
  /**
   * Language to the file on disk that holds it, which is what an edit has to be written back to.
   * A JSON source lists every language it carries against the same path.
   */
  sources: { [key in string]: string };
  entries: { [key in string]: { [key in string]: TranslationVariant | null } };
};

/**
 * Something worth reporting about a file that was opened anyway.
 *
 * The reader refuses nothing on content: an editor that will not open the file you need to fix is
 * no use, and the build and verifier keep their own guards.
 */
export type TranslationFinding = {
  rule: string;
  subject: string | null;
  message: string;
};

/** An opened translations root, whichever layout it turned out to have. */
export type TranslationProjectDescriptor = {
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
};

/** Which layout a translations root is read with. */
export type TranslationProjectMode =
  /** XRF sources: multi-language JSON and language-suffixed XML side by side in one tree. */
  | "source"
  /** Shipped gamedata: `text/<language>/*.xml`, where the directory carries the language. */
  | "gamedata";

/** One translation's text, which is a single line or a run of them joined on build. */
export type TranslationVariant = string | Array<string>;
