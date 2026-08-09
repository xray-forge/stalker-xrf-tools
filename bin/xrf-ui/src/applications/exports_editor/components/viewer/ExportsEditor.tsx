import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsViewerConditions } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerConditions";
import { ExportsViewerDialogs } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerDialogs";
import { ExportsViewerEffects } from "@/applications/exports_editor/components/viewer/exports/ExportsViewerEffects";
import { ExportsEditorMenu } from "@/applications/exports_editor/components/viewer/ExportsEditorMenu";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function ExportsEditor(): ReactElement {
  return (
    <EditorLayout
      toolbar={<EditorToolbar title={"Exports editor"} backPath={"/exports_editor"} />}
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
