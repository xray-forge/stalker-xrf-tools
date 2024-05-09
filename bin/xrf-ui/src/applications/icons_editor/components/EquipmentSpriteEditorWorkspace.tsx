import { Grid } from "@mui/material";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { ReactElement, useEffect, useState } from "react";

import { EquipmentSpritePreview } from "@/applications/icons_editor/components/EquipmentSpritePreview";
import { Optional } from "@/core/types/general";
import { EIconsEditorCommand } from "@/lib/ipc";

export function EquipmentSpriteEditorWorkspace(): ReactElement {
  const [sourceUrl, setSourceUrl] = useState<Optional<string>>(null);

  useEffect(() => {
    invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE_URI)
      .then((uri) => {
        setSourceUrl(uri as Optional<string>);
      })
      .catch(console.error);
  }, []);

  return (
    <Grid
      className={"workspace"}
      display={"flex"}
      justifyContent={"center"}
      alignItems={"center"}
      maxWidth={"100%"}
      maxHeight={"100%"}
      overflow={"auto"}
      flexGrow={1}
      padding={1}
    >
      {sourceUrl ? (
        <EquipmentSpritePreview src={convertFileSrc(sourceUrl, "stream")} width={400} height={"auto"} />
      ) : (
        "loading..."
      )}
    </Grid>
  );
}
