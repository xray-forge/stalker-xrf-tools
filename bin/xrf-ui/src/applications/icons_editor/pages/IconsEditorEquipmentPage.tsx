import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/icons_editor/components/equipment_editor/IconsEditorEquipmentOpenForm";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";

export function IconsEditorEquipmentPage({
  equipmentContext: { isReady, spriteImage } = useManager(EquipmentManager),
}): ReactElement {
  if (isReady) {
    return spriteImage.value ? <EquipmentSpriteEditor /> : <IconsEditorEquipmentOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
