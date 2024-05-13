import { Button, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { Logger, useLogger } from "@/lib/logging";

export function EquipmentSpriteEditorMenu({
  equipmentContext: { spriteImage: { isLoading, value: spriteImage }, equipmentActions } = useManager(EquipmentManager),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const log: Logger = useLogger("editor-menu");

  const onRepackAndReopenClick = useCallback(async () => {
    try {
      await equipmentActions.repackAndOpen();
    } catch (error) {
      log.error("Failed to repack and reopen DDS:", error);
    }
  }, []);

  const onReopenClick = useCallback(async () => {
    try {
      await equipmentActions.reopen();
    } catch (error) {
      log.error("Failed to reopen DDS:", error);
    }
  }, []);

  const onCloseClick = useCallback(async () => {
    await equipmentActions.close();

    navigate("/icons_editor", { replace: true });
  }, [navigate, equipmentActions]);

  return (
    <Grid display={"flex"} direction={"column"} width={240} minWidth={240} justifySelf={"stretch"} container>
      <Grid padding={3}>Descriptors: {spriteImage?.descriptors.length ?? 0}</Grid>

      <Grid flexGrow={1} container />

      <Grid margin={0} padding={"0 24px"} width={"100%"} gap={1} direction={"column"} container>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onReopenClick}>
          Reload
        </Button>

        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onRepackAndReopenClick}>
          Repack and reload
        </Button>
      </Grid>

      <Grid padding={3}>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onCloseClick}>
          Close
        </Button>
      </Grid>
    </Grid>
  );
}
