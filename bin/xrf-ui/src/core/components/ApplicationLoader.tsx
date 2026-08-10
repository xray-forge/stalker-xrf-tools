import { CircularProgress } from "@mui/material";
import { ReactElement } from "react";

import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { CenteredColumn } from "@/core/components/layout/CenteredColumn";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { BaseComponentProps } from "@/lib/dom/element-types";

import { EditorLayout } from "./editor/EditorLayout";

/**
 * Suspense fallback shown while an editor's chunk is fetched.
 */
export function ApplicationLoader({ "data-testid": dataTestId, id, className }: BaseComponentProps): ReactElement {
  useEditorBusy(true);

  return (
    <EditorLayout toolbar={<EditorToolbar backPath={"/"} isBackDisabled={true} />}>
      <CenteredColumn data-testid={dataTestId} id={id} className={className}>
        <CircularProgress />
      </CenteredColumn>
    </EditorLayout>
  );
}
