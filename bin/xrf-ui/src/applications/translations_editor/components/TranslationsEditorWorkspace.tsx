import { Grid } from "@mui/material";
import { ReactElement } from "react";

export function TranslationsEditorWorkspace(): ReactElement {
  return (
    <Grid
      className={"workspace"}
      display={"flex"}
      justifyContent={"center"}
      alignItems={"center"}
      maxWidth={"100%"}
      maxHeight={"100%"}
      flexGrow={1}
      padding={1}
    >
      todo
    </Grid>
  );
}
