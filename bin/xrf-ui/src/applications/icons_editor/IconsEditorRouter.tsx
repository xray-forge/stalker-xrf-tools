import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { IconsEditorDescriptionOpenPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionOpenPage";
import { IconsEditorDescriptionPage } from "@/applications/icons_editor/pages/IconsEditorDescriptionPage";
import { IconsEditorEquipmentOpenPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentOpenPage";
import { IconsEditorEquipmentPage } from "@/applications/icons_editor/pages/IconsEditorEquipmentPage";
import { IconsEditorNavigatorPage } from "@/applications/icons_editor/pages/IconsEditorNavigatorPage";
import { NavigationError } from "@/core/components/NavigationError";

export function IconsEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<IconsEditorNavigatorPage />} />
      <Route path={"/icons_equipment/edit"} element={<IconsEditorEquipmentPage />} />
      <Route path={"/icons_equipment"} element={<IconsEditorEquipmentOpenPage />} />
      <Route path={"/icons_description/edit"} element={<IconsEditorDescriptionPage />} />
      <Route path={"/icons_description"} element={<IconsEditorDescriptionOpenPage />} />
      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}
