import { Nullable } from "@/core/types/general";

export interface IExportsProject {
  root: string;
  declarations: Array<IExportDescriptor>;
}

export interface IExportSourceDescriptor {
  path: string;
  line: number;
  column: number;
  /** Last line of the declaration, inclusive, so its body can be fetched without parsing again. */
  endLine: number;
}

/** The source text declaring one extern, read back on demand rather than shipped with the project. */
export interface IExportSourceContent {
  name: string;
  path: string;
  line: number;
  endLine: number;
  content: string;
}

export interface IExportParameterDescriptor {
  name: string;
  typing: string;
  description: Nullable<string>;
  isOptional: boolean;
}

export interface IExportReturnDescriptor {
  typing: string;
  description: Nullable<string>;
}

export interface IExportDescriptorBase {
  name: string;
  description: Nullable<string>;
  source: IExportSourceDescriptor;
}

export interface ICallableExportDescriptor extends IExportDescriptorBase {
  kind: "callable";
  parameters: Array<IExportParameterDescriptor>;
  returns: IExportReturnDescriptor;
}

export interface IValueExportDescriptor extends IExportDescriptorBase {
  kind: "value";
  typing: string;
}

export type IExportDescriptor = ICallableExportDescriptor | IValueExportDescriptor;
