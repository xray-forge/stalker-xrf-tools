import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { TranslationsEditor } from "@/applications/translations_editor/components/TranslationsEditor";
import { TranslationsEditorOpenForm } from "@/applications/translations_editor/components/TranslationsEditorOpenForm";
import { TranslationsManager } from "@/applications/translations_editor/store/translations";

export function TranslationsEditorProjectPage(): ReactElement {
  const { isReady, project } = useInjection(TranslationsManager);

  if (isReady) {
    return project.value ? <TranslationsEditor /> : <TranslationsEditorOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
