import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { default as RefreshIcon } from "@mui/icons-material/Refresh";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";

import { EquipmentService } from "@/applications/icons_editor/store/equipment";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";
import { Logger, useLogger } from "@/lib/logging";

export function EquipmentSpriteEditorMenu(): ReactElement {
  const log: Logger = useLogger("equipment-editor-menu");

  const equipmentService: EquipmentService = useInjection(EquipmentService);

  const onRepackAndReopenClick = useCallback(async () => {
    try {
      await equipmentService.repackAndOpenProject();
    } catch (error) {
      log.error("Failed to repack and reopen DDS:", error);
    }
  }, [log, equipmentService]);

  const onReopenClick = useCallback(async () => {
    try {
      await equipmentService.reopenEquipmentProject();
    } catch (error) {
      log.error("Failed to reopen DDS:", error);
    }
  }, [log, equipmentService]);

  const actions: Array<IEditorSideMenuItem> = useMemo(() => {
    const isLoading: boolean = equipmentService.spriteImage.isLoading;

    return [
      { label: "Reload", icon: <RefreshIcon />, isDisabled: isLoading, onClick: onReopenClick },
      { label: "Repack and reload", icon: <Inventory2Icon />, isDisabled: isLoading, onClick: onRepackAndReopenClick },
    ];
  }, [equipmentService.spriteImage.isLoading, onReopenClick, onRepackAndReopenClick]);

  return <EditorSideMenu actions={actions} />;
}
