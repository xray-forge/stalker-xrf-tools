import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { IconsEditorDescriptionOpenPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionOpenPage";
import { IconsEditorEquipmentPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentPage";
import { IconsEditorNavigatorPage } from "@/applications/icons_editor/pages/IconsEditorNavigatorPage";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { NavigationError } from "@/core/components/NavigationError";

const EquipmentEditorProvider = createProvider([EquipmentManager]);

export function IconsEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<IconsEditorNavigatorPage />} />

      <Route
        path={"/icons_equipment"}
        element={
          <EquipmentEditorProvider>
            <IconsEditorEquipmentPage />
          </EquipmentEditorProvider>
        }
      />
      <Route path={"/icons_description"} element={<IconsEditorDescriptionOpenPage />} />

      <Route path={"/icons_equipment_pack"} element={<IconsEditorDescriptionOpenPage />} />
      <Route path={"/icons_equipment_unpack"} element={<IconsEditorDescriptionOpenPage />} />

      <Route path={"/icons_description_pack"} element={<IconsEditorDescriptionOpenPage />} />
      <Route path={"/icons_description_unpack"} element={<IconsEditorDescriptionOpenPage />} />

      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}
