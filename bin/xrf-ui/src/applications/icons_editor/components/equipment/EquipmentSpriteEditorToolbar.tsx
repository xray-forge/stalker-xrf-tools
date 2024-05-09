import { AppBar, Toolbar, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";

export function EquipmentSpriteEditorToolbar({
  equipmentContext: { spriteImage: { value: spriteImage } } = useManager(EquipmentManager),
}): ReactElement {
  return (
    <AppBar position={"relative"}>
      <Toolbar variant={"dense"}>
        <Typography variant={"h6"} component={"div"}>
          {spriteImage?.path ?? "equipment.dds"} ({spriteImage?.image.width}px * {spriteImage?.image?.height}px)
        </Typography>
      </Toolbar>
    </AppBar>
  );
}
