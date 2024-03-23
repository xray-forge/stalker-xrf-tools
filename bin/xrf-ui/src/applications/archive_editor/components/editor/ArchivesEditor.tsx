import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { ArchivesMenu } from "@/applications/archive_editor/components/editor/ArchivesMenu";

export function ArchivesEditor(): ReactElement {
  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"row"}
      container={true}
      flexWrap={"nowrap"}
      width={"100%"}
      height={"100%"}
    >
      <ArchivesMenu />

      <Grid width={"auto"} height={"100%"} direction={"column"} overflow={"auto"} p={2} flexGrow={1} container>
        todo
      </Grid>
    </Grid>
  );
}
