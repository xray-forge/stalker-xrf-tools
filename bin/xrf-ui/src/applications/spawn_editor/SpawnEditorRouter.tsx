import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, ContainerProviderScope, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { SpawnEditorNavigatorPage } from "@/applications/spawn_editor/pages/SpawnEditorNavigatorPage";
import { SpawnEditorPackPage } from "@/applications/spawn_editor/pages/SpawnEditorPackPage";
import { SpawnEditorPage } from "@/applications/spawn_editor/pages/SpawnEditorPage";
import { SpawnEditorUnpackPage } from "@/applications/spawn_editor/pages/SpawnEditorUnpackPage";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn/SpawnFileManager";
import { NavigationError } from "@/core/components/NavigationError";

export function SpawnEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [SpawnFileManager] }), [parent]);

  return (
    <ContainerProvider config={config} scope={ContainerProviderScope.Parent}>
      <Routes>
        <Route path={"/"} element={<SpawnEditorNavigatorPage />} />
        <Route path={"editor/*"} element={<SpawnEditorPage />} />
        <Route path={"pack"} element={<SpawnEditorPackPage />} />
        <Route path={"unpack"} element={<SpawnEditorUnpackPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
