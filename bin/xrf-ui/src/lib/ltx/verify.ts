export interface ILtxProjectVerifyError {
  at: string;
  field: string;
  section: string;
  message: string;
}

export interface ILtxProjectVerifyResult {
  duration: number;
  checked_fields: number;
  checked_sections: number;
  errors: Array<ILtxProjectVerifyError>;
  invalid_sections: number;
  skipped_sections: number;
  total_files: number;
  total_sections: number;
  valid_sections: number;
}
