export interface IArchiveFileDescriptor {
  crc: number;
  destination: string;
  extension: string;
  name: string;
  offset: number;
  sizeCompressed: number;
  sizeReal: number;
  source: string;
}

export interface IArchiveDescriptor {
  files: Record<string, IArchiveFileDescriptor>;
  outputRootPath: string;
  path: string;
}

export interface IArchiveReadPolicy {
  extensions: Array<string>;
  maximumSize: number;
  supportsCompressedFiles: boolean;
  /** Extensions decoded into a picture rather than read as text. */
  imageExtensions: Array<string>;
  maximumImageSize: number;
}

export interface IArchivesProject {
  archives: Array<IArchiveDescriptor>;
  files: Record<string, IArchiveFileDescriptor>;
  readPolicy: IArchiveReadPolicy;
  root: string;
  sizeReal: number;
}

export interface IArchiveFileReadResult {
  name: string;
  content: string;
  size: number;
}

/** What the backend reports after writing a whole archived directory to disk. */
export interface IArchiveFolderExtractResult {
  /** Archive-relative directory that was extracted, empty for the archive root. */
  prefix: string;
  destination: string;
  extractedCount: number;
  size: number;
}

/** A decoded archived texture, ready to be used as an image source. */
export interface IArchiveImagePreview {
  name: string;
  width: number;
  height: number;
  /** PNG bytes, base64 encoded by the backend. */
  base64: string;
}
