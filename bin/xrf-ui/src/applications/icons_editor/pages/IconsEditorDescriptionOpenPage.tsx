import { Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";

export function IconsEditorDescriptionOpenPage(): ReactElement {
  return (
    <Grid
      justifyContent={"safe center"}
      alignItems={"safe center"}
      direction={"column"}
      flexWrap={"nowrap"}
      container={true}
      width={"100%"}
      height={"100%"}
      padding={4}
    >
      <Grid direction={"row"} justifyContent={"center"} flexShrink={0} marginBottom={2} container item>
        <Typography>Provide description file to open</Typography>
      </Grid>

      <ApplicationBackButton disabled={false} path={"/icons_editor"} />
    </Grid>
  );
}
