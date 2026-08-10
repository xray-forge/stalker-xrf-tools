import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ExportsViewer } from "@/applications/exports-editor/components/viewer/exports/ExportsViewer";
import { ExportsService } from "@/applications/exports-editor/store/exports";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";

export function ExportsEditor(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const navigate: NavigateFunction = useNavigate();

  const declarations = exportsService.declarations.value;

  useEditorStatus(declarations ? [`${declarations.length} exports`] : []);

  const onClose = useCallback(() => {
    navigate("/exports-editor", { replace: true });

    return exportsService.closeExports();
  }, [exportsService, navigate]);

  return (
    <EditorLayout toolbar={<EditorToolbar onBack={onClose} />}>
      <ExportsViewer />
    </EditorLayout>
  );
}
