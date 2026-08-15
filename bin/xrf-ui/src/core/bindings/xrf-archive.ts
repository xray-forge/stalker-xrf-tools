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

export type ProjectReadResult = {
  name: string;
  content: string;
  size: number;
};
