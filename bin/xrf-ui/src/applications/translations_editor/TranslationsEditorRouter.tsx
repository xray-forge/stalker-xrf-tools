import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { TranslationsEditorNavigatorPage } from "@/applications/translations_editor/pages/TranslationsEditorNavigatorPage";
import { TranslationsEditorProjectPage } from "@/applications/translations_editor/pages/TranslationsEditorProjectPage";
import { TranslationsManager } from "@/applications/translations_editor/store/translations";
import { NavigationError } from "@/core/components/NavigationError";

const TranslationsEditorProvider = createProvider([TranslationsManager]);

export function TranslationsEditorRouter(): ReactElement {
  return (
    <TranslationsEditorProvider>
      <Routes>
        <Route path={"/"} element={<TranslationsEditorNavigatorPage />} />
        <Route path={"/project"} element={<TranslationsEditorProjectPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </TranslationsEditorProvider>
  );
}
