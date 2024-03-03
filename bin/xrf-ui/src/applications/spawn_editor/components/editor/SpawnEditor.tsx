import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { SpawnEditorMenu } from "@/applications/spawn_editor/components/editor/SpawnEditorMenu";

export function SpawnEditor(): ReactElement {
  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
    >
      <SpawnEditorMenu></SpawnEditorMenu>
    </Grid>
  );
}
