// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import { ArchiveAudioPreview, ArchiveImagePreview } from "@/core/bindings/types/xrf-app";
import {
  ArchiveExtractDirectoryResult,
  ArchiveExtractResult,
  ArchivePackConfig,
  ArchivePackResult,
  ArchiveUnpackResult,
} from "@/core/bindings/types/xrf-archive";
import {
  ArchiveDescriptor,
  ArchiveFileDescriptor,
  ArchiveProject,
  ArchiveProjectReadPolicy,
  ProjectReadResult,
} from "@/core/bindings/types/xrf-volume";

/** Commands */
export const archivesCommands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:archives|close_project"),
  /**
   * Hand back a packing configuration with nothing chosen yet.
   *
   * The editor starts from this rather than from its own literals, so defaults that belong to the format
   * - the volume ceiling, the skip list, the mode - have one definition, in the packer.
   */
  defaultPackConfig: () => __TAURI_INVOKE<ArchivePackConfig>("plugin:archives|default_pack_config"),
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
