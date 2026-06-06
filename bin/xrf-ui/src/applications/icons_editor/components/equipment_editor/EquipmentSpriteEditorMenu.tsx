import { Box, Button, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { Logger, useLogger } from "@/lib/logging";

export function EquipmentSpriteEditorMenu(): ReactElement {
  const log: Logger = useLogger("equipment-editor-menu");
  const navigate: NavigateFunction = useNavigate();

  const {
    spriteImage: { isLoading, value: spriteImage },
    closeEquipmentProject,
    repackAndOpenProject,
    reopenEquipmentProject,
  } = useInjection(EquipmentManager);

  const onRepackAndReopenClick = useCallback(async () => {
    try {
      await repackAndOpenProject();
    } catch (error) {
      log.error("Failed to repack and reopen DDS:", error);
    }
  }, [log, repackAndOpenProject]);

  const onReopenClick = useCallback(async () => {
    try {
      await reopenEquipmentProject();
    } catch (error) {
      log.error("Failed to reopen DDS:", error);
    }
  }, [log, reopenEquipmentProject]);

  const onCloseClick = useCallback(async () => {
    await closeEquipmentProject();

    navigate("/icons_editor", { replace: true });
  }, [navigate, closeEquipmentProject]);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: 240, minWidth: 240, justifySelf: "stretch" }}>
      <Box sx={{ padding: 3 }}>Descriptors: {spriteImage?.descriptors.length ?? 0}</Box>

      <Grid container sx={{ flexGrow: 1 }} />

      <Box sx={{ display: "flex", margin: 0, padding: "0 24px", width: "100%", gap: 1, flexDirection: "column" }}>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onReopenClick}>
          Reload
        </Button>

        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onRepackAndReopenClick}>
          Repack and reload
        </Button>
      </Box>

      <Box sx={{ padding: 3 }}>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onCloseClick}>
          Close
        </Button>
      </Box>
    </Box>
  );
}
