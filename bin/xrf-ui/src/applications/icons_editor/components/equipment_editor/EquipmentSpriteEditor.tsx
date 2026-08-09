import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentSpriteEditorMenu } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorWorkspace";
import { EquipmentService } from "@/applications/icons_editor/store/equipment";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function EquipmentSpriteEditor(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const spriteImage = equipmentService.spriteImage.value;

  const subtitle: string = spriteImage
    ? `${spriteImage.path} (${spriteImage.image.width}px * ${spriteImage.image.height}px), ` +
      `${spriteImage.descriptors.length} descriptors`
    : "";

  return (
    <EditorLayout
      toolbar={<EditorToolbar title={"Icons editor"} subtitle={subtitle} backPath={"/icons_editor"} />}
      menu={<EquipmentSpriteEditorMenu />}
    >
      <EquipmentSpriteEditorWorkspace />
    </EditorLayout>
  );
}
