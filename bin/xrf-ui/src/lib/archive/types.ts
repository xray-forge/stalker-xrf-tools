export interface IArchiveFileReplicationDescriptor {
  crc: number;
  destination: string;
  name: string;
  offset: number;
  sizeCompressed: number;
  sizeReal: number;
  source: string;
}

export interface IArchiveFileDescriptor {
  crc: number;
  name: string;
  offset: number;
  sizeCompressed: number;
  sizeReal: number;
}

export interface IArchiveDescriptor {
  files: Record<string, IArchiveFileDescriptor>;
  outputRootPath: string;
  path: string;
}

export interface IArchivesProject {
  archives: Array<IArchiveDescriptor>;
  files: Record<string, IArchiveFileReplicationDescriptor>;
}

export interface IArchiveFileReadResult {
  name: string;
  content: string;
  size: number;
}
