import { Nullable } from "@/core/types/general";

export interface IExportParameterDescriptor {
  name: string;
  typing: string;
  comment: Nullable<string>;
}

export interface IExportDescriptor {
  filename: string;
  name: string;
  comment: Nullable<string>;
  parameters: Array<IExportParameterDescriptor>;
  typing: Nullable<string>;
  line: number;
  col: number;
}

export type TExportsDeclarations = Array<IExportDescriptor>;
