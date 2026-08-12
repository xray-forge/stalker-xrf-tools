import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { TranslationsEditor } from "@/applications/translations/components/TranslationsEditor";
import { TranslationsEditorOpenForm } from "@/applications/translations/components/TranslationsEditorOpenForm";
import { TranslationsService } from "@/applications/translations/store/translations";

/** Picker until a project is open, editor once it is. */
export function TranslationsApplicationScreen(): ReactElement {
  const translationsService: TranslationsService = useInjection(TranslationsService);

  if (translationsService.isReady) {
    return translationsService.project.value ? <TranslationsEditor /> : <TranslationsEditorOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
