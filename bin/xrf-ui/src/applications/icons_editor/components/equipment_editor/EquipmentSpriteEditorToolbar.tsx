import { AppBar, Toolbar, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentService } from "@/applications/icons_editor/store/equipment";

export function EquipmentSpriteEditorToolbar(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const spriteImage = equipmentService.spriteImage.value;

  return (
    <AppBar position={"relative"}>
      <Toolbar variant={"dense"}>
        <Typography variant={"h6"} component={"div"}>
          {spriteImage?.path ?? "equipment_editor.dds"} ({spriteImage?.image.width}px * {spriteImage?.image?.height}px)
        </Typography>
      </Toolbar>
    </AppBar>
  );
}
