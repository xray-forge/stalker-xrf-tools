// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeProject: () => __TAURI_INVOKE<null>("plugin:archives|close_project"),
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

export type ArchiveExtractDirectoryResult = {
  prefix: string;
  destination: string;
  extractedCount: number;
  size: number;
};

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

export type ProjectReadResult = {
  name: string;
  content: string;
  size: number;
};
