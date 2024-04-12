import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { ArchivesFileContent } from "@/applications/archive_editor/components/editor/ArchivesFileContent";
import { ArchivesMenu } from "@/applications/archive_editor/components/editor/ArchivesMenu";

export function ArchivesEditor(): ReactElement {
  return (
    <Grid
      alignItems={"center"}
      container={true}
      direction={"row"}
      flexWrap={"nowrap"}
      height={"100%"}
      justifyContent={"center"}
      width={"100%"}
    >
      <ArchivesMenu />
      <ArchivesFileContent />
    </Grid>
  );
}
