import { default as RefreshIcon } from "@mui/icons-material/Refresh";
import { IconButton, Tooltip } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { format } from "date-fns";
import { ReactElement, useCallback, useEffect } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentSpriteEditorMenu } from "@/applications/icons-editor/components/equipment-editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons-editor/components/equipment-editor/EquipmentSpriteEditorWorkspace";
import { EquipmentService } from "@/applications/icons-editor/store/equipment";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { Nullable } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function EquipmentSpriteEditor(): ReactElement {
  const log: Logger = useLogger("equipment-editor");

  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const spriteImage = equipmentService.spriteImage.value;

  const navigate: NavigateFunction = useNavigate();

  const isLoading: boolean = equipmentService.spriteImage.isLoading;
  const repackedAt: Nullable<number> = equipmentService.repackedAt;

  const onReload = useCallback(async () => {
    try {
      await equipmentService.reopenEquipmentProject();
    } catch (error) {
      // Published as the sprite failure and rendered by the menu. Logged here for the stack.
      log.error("Failed to reload DDS:", error);
    }
  }, [log, equipmentService]);

  const onClose = useCallback(async () => {
    await equipmentService.closeEquipmentProject();

    navigate("/icons-editor", { replace: true });
  }, [navigate, equipmentService]);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent): void {
      if (event.key === "F5" && !isLoading) {
        event.preventDefault();
        void onReload();
      }
    }

    window.addEventListener("keydown", onKeyDown);

    return () => window.removeEventListener("keydown", onKeyDown);
  }, [isLoading, onReload]);

  useEditorStatus(
    spriteImage
      ? [
          `${spriteImage.image.width} x ${spriteImage.image.height}`,
          `${spriteImage.descriptors.length} descriptors`,
          ...(repackedAt ? [`Repacked ${format(repackedAt, "HH:mm")}`] : []),
        ]
      : []
  );

  useEditorBusy(isLoading);

  return (
    <EditorLayout
      toolbar={
        <EditorToolbar
          subtitle={spriteImage?.path}
          isBackDisabled={isLoading}
          actions={
            <Tooltip describeChild title={"Reload sprite (F5)"}>
              <span>
                <IconButton aria-label={"Reload sprite"} color={"inherit"} disabled={isLoading} onClick={onReload}>
                  <RefreshIcon fontSize={"small"} />
                </IconButton>
              </span>
            </Tooltip>
          }
          onBack={onClose}
        />
      }
      menu={<EquipmentSpriteEditorMenu />}
    >
      <EquipmentSpriteEditorWorkspace />
    </EditorLayout>
  );
}
