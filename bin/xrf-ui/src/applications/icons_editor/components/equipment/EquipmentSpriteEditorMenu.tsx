import { Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";

export function EquipmentSpriteEditorMenu({
  equipmentContext: { spriteImage: { value: spriteImage } } = useManager(EquipmentManager),
}): ReactElement {
  return (
    <Grid width={200} minWidth={200} justifySelf={"stretch"}>
      <Grid padding={3}>
        <Typography>Size:</Typography>
        {spriteImage?.image.width}px * {spriteImage?.image?.height}px
      </Grid>
    </Grid>
  );
}
