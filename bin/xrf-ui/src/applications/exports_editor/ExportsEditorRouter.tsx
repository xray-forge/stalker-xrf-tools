import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsEditorNavigatorPage } from "@/applications/exports_editor/pages/ExportsEditorNavigatorPage";
import { ExportsEditorPage } from "@/applications/exports_editor/pages/ExportsEditorPage";
import { ExportsManager } from "@/applications/exports_editor/store/exports";
import { NavigationError } from "@/core/components/NavigationError";

const ExportsViewerProviders = createProvider([ExportsManager]);

export function ExportsEditorRouter(): ReactElement {
  return (
    <ExportsViewerProviders>
      <Routes>
        <Route path={"/"} element={<ExportsEditorNavigatorPage />} />
        <Route path={"/exports/*"} element={<ExportsEditorPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ExportsViewerProviders>
  );
}
