// Auto-generated rust bindings. Do not edit it manually.

export type ArchiveDescriptor = {
  createdAt: number | null;
  modifiedAt: number | null;
  files: { [key in string]: ArchiveFileDescriptor };
  outputRootPath: string;
  path: string;
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
 * One volume set at a path the caller names, merged into a single name table.
 *
 * Scoped to a path on purpose: which directories of an installation hold volumes is a question the mount planner in
 * `xrf-vfs` answers (`XrayMountPlan::from_fsgame`), and answering it here too would put `fsgame.ltx` knowledge in the
 * volume-format layer and give the same declaration two readers.
 *
 * Later volumes win the merge, so a patch volume shadows the entry it replaces.
 */
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

export type ProjectReadResult = {
  name: string;
  content: string;
  size: number;
};
