import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { TranslationsEditorMenu } from "@/applications/translations_editor/components/TranslationsEditorMenu";
import { TranslationsEditorToolbar } from "@/applications/translations_editor/components/TranslationsEditorToolbar";
import { TranslationsEditorWorkspace } from "@/applications/translations_editor/components/TranslationsEditorWorkspace";

export function TranslationsEditor(): ReactElement {
  return (
    <Grid
      direction={"column"}
      justifyContent={"center"}
      alignItems={"center"}
      width={"100%"}
      height={"100%"}
      wrap={"nowrap"}
      container
    >
      <TranslationsEditorToolbar />

      <Grid flexGrow={1} flexWrap={"nowrap"} container>
        <TranslationsEditorMenu />
        <TranslationsEditorWorkspace />
      </Grid>
    </Grid>
  );
}
