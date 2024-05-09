import { Button, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";

export function EquipmentSpriteEditorMenu({
  equipmentContext: { spriteImage: { isLoading, value: spriteImage }, equipmentActions } = useManager(EquipmentManager),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onCloseClick = useCallback(async () => {
    await equipmentActions.close();

    navigate("/icons_editor", { replace: true });
  }, [navigate, equipmentActions]);

  return (
    <Grid display={"flex"} direction={"column"} width={240} minWidth={240} justifySelf={"stretch"} container>
      <Grid padding={3}>Descriptors: {spriteImage?.descriptors.length ?? 0}</Grid>

      <Grid flexGrow={1} container />

      <Grid padding={3}>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onCloseClick}>
          Close
        </Button>
      </Grid>
    </Grid>
  );
}
