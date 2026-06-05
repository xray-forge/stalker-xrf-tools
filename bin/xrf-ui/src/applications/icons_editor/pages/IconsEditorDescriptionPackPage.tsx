import { Box, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";

export function IconsEditorDescriptionPackPage(): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "safe center",
        alignItems: "safe center",
        flexDirection: "column",
        flexWrap: "nowrap",
        width: "100%",
        height: "100%",
        padding: 4,
      }}
    >
      <Grid container sx={{ justifyContent: "center", flexShrink: 0, marginBottom: 2 }}>
        <Typography>Provide description file to pack</Typography>
      </Grid>

      <ApplicationBackButton disabled={false} path={"/icons_editor"} />
    </Box>
  );
}
