import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/equipment-icons-editor/components/equipment-editor/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/equipment-icons-editor/components/equipment-editor/IconsEditorEquipmentOpenForm";
import { EquipmentService } from "@/core/equipment-icons";
import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";

/** Picker until a sprite is open, editor once it is. */
export function EquipmentIconsEditorApplication(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  if (equipmentService.isReady) {
    return equipmentService.spriteImage.value ? <EquipmentSpriteEditor /> : <IconsEditorEquipmentOpenForm />;
  }

  return <ApplicationLoader />;
}
