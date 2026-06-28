import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/icons_editor/components/equipment_editor/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/icons_editor/components/equipment_editor/IconsEditorEquipmentOpenForm";
import { EquipmentService } from "@/applications/icons_editor/store/equipment";

export function IconsEditorEquipmentPage(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  if (equipmentService.isReady) {
    return equipmentService.spriteImage.value ? <EquipmentSpriteEditor /> : <IconsEditorEquipmentOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
