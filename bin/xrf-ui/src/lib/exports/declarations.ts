import { Optional } from "@/core/types/general";

export interface IExportParameterDescriptor {
  name: string;
  typing: string;
  comment: Optional<string>;
}

export interface IExportDescriptor {
  filename: string;
  name: string;
  comment: Optional<string>;
  parameters: Array<IExportParameterDescriptor>;
  typing: Optional<string>;
  line: number;
  col: number;
}

export type TExportsDeclarations = Array<IExportDescriptor>;
