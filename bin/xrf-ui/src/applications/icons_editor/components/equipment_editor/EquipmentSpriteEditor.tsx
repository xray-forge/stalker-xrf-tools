import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { EquipmentSpriteEditorMenu } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorToolbar } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorToolbar";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorWorkspace";

export function EquipmentSpriteEditor(): ReactElement {
  return (
    <Grid
      direction={"column"}
      justifyContent={"center"}
      alignItems={"center"}
      width={"100%"}
      height={"100%"}
      wrap={"nowrap"}
      container
    >
      <EquipmentSpriteEditorToolbar />

      <Grid flexGrow={1} flexWrap={"nowrap"} width={"100%"} container>
        <EquipmentSpriteEditorMenu />
        <EquipmentSpriteEditorWorkspace />
      </Grid>
    </Grid>
  );
}
