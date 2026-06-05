import { Box, Grid } from "@mui/material";
import { ReactElement } from "react";

import { EquipmentSpriteEditorMenu } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorMenu";
import { EquipmentSpriteEditorToolbar } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorToolbar";
import { EquipmentSpriteEditorWorkspace } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditorWorkspace";

export function EquipmentSpriteEditor(): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        width: "100%",
        height: "100%",
        flexWrap: "nowrap",
      }}
    >
      <EquipmentSpriteEditorToolbar />

      <Grid container wrap={"nowrap"} sx={{ flexGrow: 1, width: "100%" }}>
        <EquipmentSpriteEditorMenu />
        <EquipmentSpriteEditorWorkspace />
      </Grid>
    </Box>
  );
}
