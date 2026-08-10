import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsEditorNavigatorPage } from "@/applications/exports-editor/pages/ExportsEditorNavigatorPage";
import { ExportsEditorPage } from "@/applications/exports-editor/pages/ExportsEditorPage";
import { ExportsService } from "@/applications/exports-editor/store/exports";
import { NavigationError } from "@/core/components/NavigationError";

export function ExportsEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [ExportsService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <Routes>
        <Route path={"/"} element={<ExportsEditorNavigatorPage />} />
        <Route path={"/exports/*"} element={<ExportsEditorPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
