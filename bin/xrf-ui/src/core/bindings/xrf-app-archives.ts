// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:archives|close_project"),
  /**
   * Write the selection rules of a configuration out as an xrCompress configuration file.
   *
   * Only what such a file can carry is written, so a round trip through import returns what was exported.
   * Paths, name, mode, and volume size belong to the run rather than to the file.
   */
  exportPackConfig: (path: string, config: ArchivePackConfig) =>
    __TAURI_INVOKE<null>("plugin:archives|export_pack_config", { path, config }),
  /**
   * Read an xrCompress configuration file over the configuration the caller holds.
   *
   * Layers rather than replaces, matching how the command line applies `--ltx`: a configuration file
   * carries selection rules and a header, never the source, destination, name, mode, or volume size, so
   * those stay as the caller had them.
   */
  importPackConfig: (path: string, config: ArchivePackConfig) =>
    __TAURI_INVOKE<ArchivePackConfig>("plugin:archives|import_pack_config", { path, config }),
  /** Write a single archived file to a path the user chose. */
  extractFile: (name: string, destination: string) =>
    __TAURI_INVOKE<ArchiveExtractResult>("plugin:archives|extract_file", { name, destination }),
  /**
   * Write every archived file under one directory into a destination root.
   *
   * An empty prefix means the whole archive, so this also covers extracting everything without needing
   * a separate command.
   */
  extractDirectory: (prefix: string, destination: string) =>
    __TAURI_INVOKE<ArchiveExtractDirectoryResult>("plugin:archives|extract_directory", { prefix, destination }),
  getProject: () =>
    __TAURI_INVOKE<{
      archives: Array<ArchiveDescriptor>;
      files: { [key in string]: ArchiveFileDescriptor };
      readPolicy: ArchiveProjectReadPolicy;
      root: string;
      sizeReal: number;
    } | null>("plugin:archives|get_project"),
  hasProject: () => __TAURI_INVOKE<boolean>("plugin:archives|has_project"),
  openProject: (path: string) => __TAURI_INVOKE<ArchiveProject>("plugin:archives|open_project", { path }),
  /**
   * Pack a directory into archive volumes from a configuration held by the caller.
   *
   * Takes the whole configuration rather than a file path, so the editor packs exactly what is on screen
   * without having to save it first.
   */
  packDirectory: (config: ArchivePackConfig) =>
    __TAURI_INVOKE<ArchivePackResult>("plugin:archives|pack_directory", { config }),
  /** Hand an archived sound to the webview, along with whatever the engine would read from it. */
  readAudio: (path: string) => __TAURI_INVOKE<ArchiveAudioPreview>("plugin:archives|read_audio", { path }),
  readFile: (path: string) => __TAURI_INVOKE<ProjectReadResult>("plugin:archives|read_file", { path }),
  /**
   * Decode an archived DDS into a PNG the webview can display.
   *
   * Compressed entries are fine here, unlike the text preview: the bytes are decompressed on the way out
   * of the archive, so compression is invisible by the time there is an image to decode.
   */
  readImage: (path: string) => __TAURI_INVOKE<ArchiveImagePreview>("plugin:archives|read_image", { path }),
  unpackDirectory: (from: string, destination: string) =>
    __TAURI_INVOKE<ArchiveUnpackResult>("plugin:archives|unpack_directory", { from, destination }),
};

/* Types */
/** The X-Ray source parameters carried in a sound's first vorbis comment. */
export type ArchiveAudioParameters = {
  minDistance: number | null;
  maxDistance: number | null;
  baseVolume: number | null;
  gameType: number;
  maxAiDistance: number | null;
};

export type ArchiveAudioPreview = {
  name: string;
  channels: number;
  sampleRate: number;
  /** Absent for a sound carrying no recognized X-Ray comment, where the engine uses its own defaults. */
  parameters: ArchiveAudioParameters | null;
  /** The ogg bytes as stored, base64 encoded. The webview decodes vorbis itself. */
  base64: string;
};

export type ArchiveDescriptor = {
  createdAt: number | null;
  modifiedAt: number | null;
  files: { [key in string]: ArchiveFileDescriptor };
  outputRootPath: string;
  path: string;
};

/** What extracting one archived directory produced. */
export type ArchiveExtractDirectoryResult = {
  prefix: string;
  destination: string;
  extractedCount: number;
  size: number;
};

/** What extracting one archived file produced. */
export type ArchiveExtractResult = {
  name: string;
  destination: string;
  size: number;
};

export type ArchiveFileDescriptor = {
  crc: number;
  source: string;
  destination: string;
  extension: string;
  name: string;
  offset: number;
  sizeCompressed: number;
  sizeReal: number;
};

export type ArchiveImagePreview = {
  name: string;
  width: number;
  height: number;
  /** PNG bytes, base64 encoded so the webview can use them directly as an image source. */
  base64: string;
};

/**
 * Everything needed to pack one archive volume set.
 *
 * Built from defaults, then optionally from an xrCompress LTX, then from explicit parameters, so a
 * command line and a form can layer over the same config file in the same order.
 *
 * Also the wire contract the desktop editor holds: it is read from a configuration file, edited in
 * place, packed, and written back, so all three surfaces speak one shape.
 */
export type ArchivePackConfig = {
  /** Root the archived names are relative to, normally a `gamedata` directory. */
  source: string;
  destination: string;
  /** Base name of the volumes, which become `<name>.db0`, `<name>.db1` and so on. */
  name: string;
  includeFiles: Array<string>;
  includeFolders: Array<ArchivePackFolder>;
  excludeFolders: Array<ArchivePackFolder>;
  /** Extension patterns from `[options] exclude_exts`, matched against the extension with its dot. */
  excludeExtensions: Array<string>;
  /** Apply the skip rules xrCompress hard-codes for editor and source leftovers. */
  isWithSkipList: boolean;
  /** Verbatim `[header]` text written as chunk 666. */
  header: string | null;
  mode: ArchivePackMode;
  maxVolumeSize: number;
  volumeExtension: ArchiveVolumeExtension;
};

/**
 * One `[include_folders]` or `[exclude_folders]` entry.
 *
 * The boolean has a different meaning on each side, which is an xrCompress quirk worth stating: an
 * included folder recurses into subfolders, while an excluded one matches by prefix rather than exactly.
 */
export type ArchivePackFolder = {
  path: string;
  isRecursive: boolean;
};

/** How file payloads are stored in the archive. */
export type ArchivePackMode =
  /** Compress what the engine expects to be compressed and store the rest. */
  | "Compress"
  /** Store everything, the `-store` flag of xrCompress. */
  | "Store";

/** What one packing run produced. */
export type ArchivePackResult = {
  /** Volumes written, in mount order. */
  volumes: Array<string>;
  filesTotal: number;
  /** Files the include, exclude, and skip rules left out. */
  filesSkipped: number;
  filesStored: number;
  filesCompressed: number;
  /** Files that shared an identical earlier payload and cost only a descriptor row. */
  filesAliased: number;
  sizeSource: number;
  sizeWritten: number;
  duration: number;
};

export type ArchiveProject = {
  archives: Array<ArchiveDescriptor>;
  files: { [key in string]: ArchiveFileDescriptor };
  readPolicy: ArchiveProjectReadPolicy;
  root: string;
  sizeReal: number;
};

export type ArchiveProjectReadPolicy = {
  extensions: Array<string>;
  maximumSize: number;
  /** Extensions decoded into a picture. Compression does not apply: it is undone before decoding. */
  imageExtensions: Array<string>;
  maximumImageSize: number;
  /** Extensions played by the webview itself, so the backend only has to hand over the bytes. */
  audioExtensions: Array<string>;
  maximumAudioSize: number;
};

export type ArchiveUnpackResult = {
  archives: Array<string>;
  duration: number;
  destination: string;
  prepareDuration: number;
  unpackedSize: number;
  unpackDuration: number;
};

/** Extension the produced volumes carry, which also decides how the engine treats a missing header. */
export type ArchiveVolumeExtension = "Db" | "Xdb";

export type ProjectReadResult = {
  name: string;
  content: string;
  size: number;
};
