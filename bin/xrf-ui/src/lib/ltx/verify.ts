export interface ILtxProjectVerifyError {
  at: string;
  field: string;
  message: string;
  section: string;
}

export interface ILtxProjectVerifyResult {
  checkedFields: number;
  checkedSections: number;
  duration: number;
  errors: Array<ILtxProjectVerifyError>;
  invalidSections: number;
  skippedSections: number;
  totalFiles: number;
  totalSections: number;
  validSections: number;
}
