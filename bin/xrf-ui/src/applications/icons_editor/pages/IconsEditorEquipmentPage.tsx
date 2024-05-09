import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/icons_editor/components/equipment/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/icons_editor/components/equipment/IconsEditorEquipmentOpenForm";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";

export function IconsEditorEquipmentPage({
  equipmentContext: { spriteImage } = useManager(EquipmentManager),
}): ReactElement {
  if (spriteImage.value) {
    return <EquipmentSpriteEditor />;
  }

  return <IconsEditorEquipmentOpenForm />;
}
