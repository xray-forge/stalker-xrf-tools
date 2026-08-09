import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentSpriteEditorMenu } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorWorkspace";
import { EquipmentService } from "@/applications/icons_editor/store/equipment";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function EquipmentSpriteEditor(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const spriteImage = equipmentService.spriteImage.value;

  const navigate: NavigateFunction = useNavigate();

  const subtitle: string = spriteImage
    ? `${spriteImage.path} (${spriteImage.image.width}px * ${spriteImage.image.height}px), ` +
      `${spriteImage.descriptors.length} descriptors`
    : "";

  const onClose = useCallback(async () => {
    await equipmentService.closeEquipmentProject();

    navigate("/icons_editor", { replace: true });
  }, [navigate, equipmentService]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar title={"Icons editor"} subtitle={subtitle} onBack={onClose} />}
      menu={<EquipmentSpriteEditorMenu />}
    >
      <EquipmentSpriteEditorWorkspace />
    </EditorLayout>
  );
}
