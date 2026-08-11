import { Nullable } from "@/core/types/general";

export interface IExportsProject {
  root: string;
  declarations: Array<IExportDescriptor>;
}

export interface IExportSourceDescriptor {
  path: string;
  line: number;
  column: number;
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
