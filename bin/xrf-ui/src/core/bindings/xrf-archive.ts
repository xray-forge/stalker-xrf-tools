// Auto-generated rust bindings. Do not edit it manually.

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
