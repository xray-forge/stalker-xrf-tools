import { Box, Button, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { TranslationsService } from "@/applications/translations_editor/store/translations";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditorMenu(): ReactElement {
  const log: Logger = useLogger("translations-editor-menu");
  const navigate: NavigateFunction = useNavigate();

  const translationsService: TranslationsService = useInjection(TranslationsService);

  const onCloseClick = useCallback(async () => {
    log.info("Closing translations");

    await translationsService.closeTranslationsProject();

    navigate("/translations_editor", { replace: true });
  }, [log, navigate, translationsService]);

  if (!translationsService.project.value) {
    throw new Error("Unexpected rendering of translations menu.");
  }

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: 240, minWidth: 240, justifySelf: "stretch" }}>
      <Box sx={{ padding: 3 }}>{Object.keys(translationsService.project.value).length} files</Box>

      <Grid container sx={{ flexGrow: 1 }} />

      <Box sx={{ display: "flex", margin: 0, padding: "0 24px", width: "100%", gap: 1, flexDirection: "column" }}>
        todo
      </Box>

      <Box sx={{ padding: 3 }}>
        <Button
          fullWidth={true}
          variant={"outlined"}
          disabled={translationsService.project.isLoading}
          onClick={onCloseClick}
        >
          Close
        </Button>
      </Box>
    </Box>
  );
}
