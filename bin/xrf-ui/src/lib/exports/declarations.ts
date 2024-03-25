import { Optional } from "@/core/types/general";

export interface IExportParameterDescriptor {
  name: string;
  typing: string;
  comment: Optional<string>;
}

export interface IExportDescriptor {
  name: string;
  comment: Optional<string>;
  parameters: Array<IExportParameterDescriptor>;
}

export interface IExportsDeclarations {
  conditions: Array<IExportDescriptor>;
  dialogs: Array<IExportDescriptor>;
  effects: Array<IExportDescriptor>;
}
