import { IExportDescriptor, IExportParameterDescriptor } from "@/lib/exports";

export function formatExportSignature(declaration: IExportDescriptor): string {
  switch (declaration.kind) {
    case "callable":
      return `${declaration.name}(${declaration.parameters
        .map(
          (parameter: IExportParameterDescriptor) =>
            `${parameter.name}${parameter.isOptional ? "?" : ""}: ${parameter.typing}`
        )
        .join(", ")}): ${declaration.returns.typing}`;

    case "value":
      return `${declaration.name}: ${declaration.typing}`;

    default: {
      return declaration as never;
    }
  }
}
