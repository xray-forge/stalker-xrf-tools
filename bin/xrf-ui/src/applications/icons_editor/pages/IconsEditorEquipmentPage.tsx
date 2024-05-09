import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { EquipmentSpriteEditorMenu } from "@/applications/icons_editor/components/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorToolbar } from "@/applications/icons_editor/components/EquipmentSpriteEditorToolbar";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons_editor/components/EquipmentSpriteEditorWorkspace";

export function IconsEditorEquipmentPage(): ReactElement {
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

      <Grid flexGrow={1} flexWrap={"nowrap"} container>
        <EquipmentSpriteEditorMenu />
        <EquipmentSpriteEditorWorkspace />
      </Grid>
    </Grid>
  );
}
