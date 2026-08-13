import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/equipment-icons/components/equipment-editor/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/equipment-icons/components/equipment-editor/IconsEditorEquipmentOpenForm";
import { ApplicationLoader } from "@/core/components/ApplicationLoader";
import { EquipmentService } from "@/core/equipment-icons";

/** Picker until a sprite is open, editor once it is. */
export function EquipmentIconsApplication(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  if (equipmentService.isReady) {
    return equipmentService.spriteImage.value ? <EquipmentSpriteEditor /> : <IconsEditorEquipmentOpenForm />;
  }

  return <ApplicationLoader />;
}
