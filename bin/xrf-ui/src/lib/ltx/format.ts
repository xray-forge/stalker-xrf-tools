export interface ILtxProjectFormatResult {
  duration: number;
  invalid_files: number;
  to_format: Array<string>;
  total_files: number;
  valid_files: number;
}
