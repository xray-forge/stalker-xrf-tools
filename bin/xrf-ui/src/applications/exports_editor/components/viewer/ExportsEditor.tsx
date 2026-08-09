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
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";

export function ExportsEditor(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const navigate: NavigateFunction = useNavigate();

  const declarations = exportsService.declarations.value;

  useEditorStatus(
    declarations
      ? [
        `${declarations.conditions.length} conditions`,
        `${declarations.dialogs.length} dialogs`,
        `${declarations.effects.length} effects`,
      ]
      : []
  );

  const onClose = useCallback(() => {
    navigate("/exports_editor", { replace: true });

    return exportsService.closeExports();
  }, [exportsService, navigate]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar onBack={onClose} />}
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
