import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, Route, Routes, useNavigate } from "react-router-dom";

import { ExportsViewerConditions } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerConditions";
import { ExportsViewerDialogs } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerDialogs";
import { ExportsViewerEffects } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerEffects";
import { ExportsEditorMenu } from "@/applications/exports_editor/components/viewer/ExportsEditorMenu";
import { ExportsService } from "@/applications/exports_editor/store/exports";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function ExportsEditor(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const navigate: NavigateFunction = useNavigate();

  const onClose = useCallback(() => {
    navigate("/exports_editor", { replace: true });

    return exportsService.closeExports();
  }, [exportsService, navigate]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar title={"Exports editor"} onBack={onClose} />}
      menu={<ExportsEditorMenu />}
    >
      <Routes>
        <Route path={"/conditions"} element={<ExportsViewerConditions />} />
        <Route path={"/effects"} element={<ExportsViewerEffects />} />
        <Route path={"/dialogs"} element={<ExportsViewerDialogs />} />
        <Route path={"/*"} element={<ExportsViewerConditions />} />
      </Routes>
    </EditorLayout>
  );
}
