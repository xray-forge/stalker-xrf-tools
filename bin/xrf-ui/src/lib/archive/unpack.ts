export interface IArchiveUnpackResult {
  archives: Array<string>;
  destination: string;
  duration: number;
  prepareDuration: number;
  unpackDuration: number;
  unpackedSize: number;
}
