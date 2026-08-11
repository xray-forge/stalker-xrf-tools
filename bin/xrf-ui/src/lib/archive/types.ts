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
