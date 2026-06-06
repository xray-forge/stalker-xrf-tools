import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, ContainerProviderScope, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { TranslationsEditorNavigatorPage } from "@/applications/translations_editor/pages/TranslationsEditorNavigatorPage";
import { TranslationsEditorProjectPage } from "@/applications/translations_editor/pages/TranslationsEditorProjectPage";
import { TranslationsManager } from "@/applications/translations_editor/store/translations";
import { NavigationError } from "@/core/components/NavigationError";

export function TranslationsEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [TranslationsManager] }), [parent]);

  return (
    <ContainerProvider config={config} scope={ContainerProviderScope.Parent}>
      <Routes>
        <Route path={"/"} element={<TranslationsEditorNavigatorPage />} />
        <Route path={"/project"} element={<TranslationsEditorProjectPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
