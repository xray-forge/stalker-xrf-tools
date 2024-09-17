import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsManager } from "@/applications/exports_viewer/store/exports";
import { TranslationsEditorNavigatorPage } from "@/applications/translations_editor/pages/TranslationsEditorNavigatorPage";

const ExportsViewerProviders = createProvider([ExportsManager]);

export function TranslationsEditorRouter(): ReactElement {
  return (
    <ExportsViewerProviders>
      <Routes>
        <Route path={"/"} element={<TranslationsEditorNavigatorPage />} />
      </Routes>
    </ExportsViewerProviders>
  );
}
