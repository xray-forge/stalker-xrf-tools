// Auto-generated rust bindings. Do not edit it manually.

import { XrfError } from "@/core/bindings/types/xrf-error";

export type LtxProjectFormatResult = {
  duration: number;
  invalidFiles: number;
  toFormat: Array<string>;
  totalFiles: number;
  validFiles: number;
};

export type LtxProjectVerifyResult = {
  checkedFields: number;
  checkedSections: number;
  duration: number;
  errors: Array<XrfError>;
  invalidSections: number;
  skippedSections: number;
  totalFiles: number;
  totalSections: number;
  validSections: number;
};
