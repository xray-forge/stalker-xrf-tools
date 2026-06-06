import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, ContainerProviderScope, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { ArchivesEditorNavigatorPage } from "@/applications/archive_editor/pages/ArchivesEditorNavigatorPage";
import { ArchivesEditorPage } from "@/applications/archive_editor/pages/ArchivesEditorPage";
import { ArchivesEditorUnpackerPage } from "@/applications/archive_editor/pages/ArchivesEditorUnpackerPage";
import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { NavigationError } from "@/core/components/NavigationError";

export function ArchivesEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [ArchivesService] }), [parent]);

  return (
    <ContainerProvider config={config} scope={ContainerProviderScope.Parent}>
      <Routes>
        <Route path={"/"} element={<ArchivesEditorNavigatorPage />} />
        <Route path={"/editor"} element={<ArchivesEditorPage />} />
        <Route path={"/explorer"} element={<ArchivesEditorNavigatorPage />} />
        <Route path={"/unpacker"} element={<ArchivesEditorUnpackerPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
