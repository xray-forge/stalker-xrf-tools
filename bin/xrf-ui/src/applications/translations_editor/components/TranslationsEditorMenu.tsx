import { Button, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { TranslationsManager } from "@/applications/translations_editor/store/translations";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditorMenu({
  equipmentContext: { project: { isLoading, value: translationsProject }, translationsActions } = useManager(
    TranslationsManager
  ),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const log: Logger = useLogger("translations-editor-menu");

  const onCloseClick = useCallback(async () => {
    log.info("Closing translations");

    await translationsActions.close();

    navigate("/translations_editor", { replace: true });
  }, [navigate, translationsActions]);

  if (!translationsProject) {
    throw new Error("Unexpected rendering of translations menu.");
  }

  return (
    <Grid display={"flex"} direction={"column"} width={240} minWidth={240} justifySelf={"stretch"} container>
      <Grid padding={3}>{Object.keys(translationsProject).length} files</Grid>

      <Grid flexGrow={1} container />

      <Grid margin={0} padding={"0 24px"} width={"100%"} gap={1} direction={"column"} container>
        todo
      </Grid>

      <Grid padding={3}>
        <Button fullWidth={true} variant={"outlined"} disabled={isLoading} onClick={onCloseClick}>
          Close
        </Button>
      </Grid>
    </Grid>
  );
}
