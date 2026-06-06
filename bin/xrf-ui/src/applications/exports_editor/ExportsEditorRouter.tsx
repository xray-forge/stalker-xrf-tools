import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, ContainerProviderScope, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsEditorNavigatorPage } from "@/applications/exports_editor/pages/ExportsEditorNavigatorPage";
import { ExportsEditorPage } from "@/applications/exports_editor/pages/ExportsEditorPage";
import { ExportsManager } from "@/applications/exports_editor/store/exports";
import { NavigationError } from "@/core/components/NavigationError";

export function ExportsEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [ExportsManager] }), [parent]);

  return (
    <ContainerProvider config={config} scope={ContainerProviderScope.Parent}>
      <Routes>
        <Route path={"/"} element={<ExportsEditorNavigatorPage />} />
        <Route path={"/exports/*"} element={<ExportsEditorPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
