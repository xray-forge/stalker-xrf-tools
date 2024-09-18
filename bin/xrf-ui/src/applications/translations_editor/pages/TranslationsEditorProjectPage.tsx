import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { TranslationsEditor } from "@/applications/translations_editor/components/TranslationsEditor";
import { TranslationsEditorOpenForm } from "@/applications/translations_editor/components/TranslationsEditorOpenForm";
import { TranslationsManager } from "@/applications/translations_editor/store/translations";

export function TranslationsEditorProjectPage({
  translationsContext: { isReady, project } = useManager(TranslationsManager),
}): ReactElement {
  if (isReady) {
    return project.value ? <TranslationsEditor /> : <TranslationsEditorOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
