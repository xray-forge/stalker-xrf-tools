import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { TranslationsEditorNavigatorPage } from "@/applications/translations_editor/pages/TranslationsEditorNavigatorPage";

export function TranslationsEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<TranslationsEditorNavigatorPage />} />
    </Routes>
  );
}
