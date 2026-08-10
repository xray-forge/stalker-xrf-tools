import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentSpriteEditorMenu } from "@/applications/icons-editor/components/equipment-editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons-editor/components/equipment-editor/EquipmentSpriteEditorWorkspace";
import { EquipmentService } from "@/applications/icons-editor/store/equipment";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";

export function EquipmentSpriteEditor(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const spriteImage = equipmentService.spriteImage.value;

  const navigate: NavigateFunction = useNavigate();

  useEditorStatus(
    spriteImage
      ? [`${spriteImage.image.width} x ${spriteImage.image.height}`, `${spriteImage.descriptors.length} descriptors`]
      : []
  );

  const onClose = useCallback(async () => {
    await equipmentService.closeEquipmentProject();

    navigate("/icons-editor", { replace: true });
  }, [navigate, equipmentService]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar subtitle={spriteImage?.path} onBack={onClose} />}
      menu={<EquipmentSpriteEditorMenu />}
    >
      <EquipmentSpriteEditorWorkspace />
    </EditorLayout>
  );
}
