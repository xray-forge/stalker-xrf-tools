import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { IconsEditorDescriptionOpenPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionOpenPage";
import { IconsEditorDescriptionPackPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionPackPage";
import { IconsEditorDescriptionUnpackPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionUnpackPage";
import { IconsEditorEquipmentPackPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentPackPage";
import { IconsEditorEquipmentPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentPage";
import { IconsEditorEquipmentUnpackPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentUnpackPage";
import { IconsEditorNavigatorPage } from "@/applications/icons_editor/pages/IconsEditorNavigatorPage";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { NavigationError } from "@/core/components/NavigationError";

const EquipmentEditorProvider = createProvider([EquipmentManager]);

export function IconsEditorRouter(): ReactElement {
  return (
    <EquipmentEditorProvider>
      <Routes>
        <Route path={"/"} element={<IconsEditorNavigatorPage />} />

        <Route path={"/icons_equipment"} element={<IconsEditorEquipmentPage />} />
        <Route path={"/icons_equipment_pack"} element={<IconsEditorEquipmentPackPage />} />
        <Route path={"/icons_equipment_unpack"} element={<IconsEditorEquipmentUnpackPage />} />

        <Route path={"/icons_description"} element={<IconsEditorDescriptionOpenPage />} />
        <Route path={"/icons_description_pack"} element={<IconsEditorDescriptionPackPage />} />
        <Route path={"/icons_description_unpack"} element={<IconsEditorDescriptionUnpackPage />} />

        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </EquipmentEditorProvider>
  );
}
