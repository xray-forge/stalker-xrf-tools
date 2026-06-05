import { Box, Grid } from "@mui/material";
import { ReactElement } from "react";

import { TranslationsEditorMenu } from "@/applications/translations_editor/components/TranslationsEditorMenu";
import { TranslationsEditorToolbar } from "@/applications/translations_editor/components/TranslationsEditorToolbar";
import { TranslationsEditorWorkspace } from "@/applications/translations_editor/components/TranslationsEditorWorkspace";

export function TranslationsEditor(): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        width: "100%",
        height: "100%",
        flexWrap: "nowrap",
      }}
    >
      <TranslationsEditorToolbar />

      <Grid container wrap={"nowrap"} sx={{ flexGrow: 1 }}>
        <TranslationsEditorMenu />
        <TranslationsEditorWorkspace />
      </Grid>
    </Box>
  );
}
