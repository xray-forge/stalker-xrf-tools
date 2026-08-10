import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { IconsEditorDescriptionOpenPage } from "@/applications/icons-editor/pages/IconsEditorDescriptionOpenPage";
import { IconsEditorDescriptionPackPage } from "@/applications/icons-editor/pages/IconsEditorDescriptionPackPage";
import { IconsEditorDescriptionUnpackPage } from "@/applications/icons-editor/pages/IconsEditorDescriptionUnpackPage";
import { IconsEditorEquipmentPackPage } from "@/applications/icons-editor/pages/IconsEditorEquipmentPackPage";
import { IconsEditorEquipmentPage } from "@/applications/icons-editor/pages/IconsEditorEquipmentPage";
import { IconsEditorEquipmentUnpackPage } from "@/applications/icons-editor/pages/IconsEditorEquipmentUnpackPage";
import { IconsEditorNavigatorPage } from "@/applications/icons-editor/pages/IconsEditorNavigatorPage";
import { EquipmentService } from "@/applications/icons-editor/store/equipment";
import { NavigationError } from "@/core/components/NavigationError";

export function IconsEditorRouter(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [EquipmentService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <Routes>
        <Route path={"/"} element={<IconsEditorNavigatorPage />} />

        <Route path={"/icons-equipment"} element={<IconsEditorEquipmentPage />} />
        <Route path={"/icons-equipment-pack"} element={<IconsEditorEquipmentPackPage />} />
        <Route path={"/icons-equipment-unpack"} element={<IconsEditorEquipmentUnpackPage />} />

        <Route path={"/icons-description"} element={<IconsEditorDescriptionOpenPage />} />
        <Route path={"/icons-description-pack"} element={<IconsEditorDescriptionPackPage />} />
        <Route path={"/icons-description-unpack"} element={<IconsEditorDescriptionUnpackPage />} />

        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ContainerProvider>
  );
}
