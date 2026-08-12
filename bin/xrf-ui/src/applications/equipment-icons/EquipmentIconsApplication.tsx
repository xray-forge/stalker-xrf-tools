import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { EquipmentSpriteEditor } from "@/applications/equipment-icons/components/equipment-editor/EquipmentSpriteEditor";
import { IconsEditorEquipmentOpenForm } from "@/applications/equipment-icons/components/equipment-editor/IconsEditorEquipmentOpenForm";
import { EquipmentService } from "@/lib/icons";

/** Picker until a sprite is open, editor once it is. */
export function EquipmentIconsApplication(): ReactElement {
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
