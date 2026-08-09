import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { VisualsEditorNavigatorPage } from "@/applications/visuals_editor/pages/VisualsEditorNavigatorPage";
import { VisualsEditorPreviewPage } from "@/applications/visuals_editor/pages/VisualsEditorPreviewPage";
import { VisualsEditorProjectPage } from "@/applications/visuals_editor/pages/VisualsEditorProjectPage";
import { NavigationError } from "@/core/components/NavigationError";

export function VisualsEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<VisualsEditorNavigatorPage />} />

      <Route path={"/visual_preview"} element={<VisualsEditorPreviewPage />} />
      <Route path={"/visual_project"} element={<VisualsEditorProjectPage />} />

      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}
