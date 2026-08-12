import { ReactElement } from "react";

import { ExportDeclarationView } from "@/applications/exports/components/viewer/exports/ExportDeclarationView";
import { ExportsViewerState } from "@/applications/exports/components/viewer/exports/ExportsViewerState";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerProps extends BaseComponentProps {
  declaration: Nullable<IExportDescriptor>;
  exportCount: number;
}

export function ExportsViewer({ declaration, exportCount }: IExportsViewerProps): ReactElement {
  if (!exportCount) {
    return (
      <ExportsViewerState
        title={"No externs found"}
        description={"This project is open, but it does not currently declare any externs."}
      />
    );
  }

  if (!declaration) {
    return (
      <ExportsViewerState
        title={"Select an export to inspect"}
        description={"Expand a namespace in the explorer and select one of its declarations."}
      />
    );
  }

  return <ExportDeclarationView declaration={declaration} />;
}
