import { Box, Button, FormHelperText, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback, useEffect } from "react";

import { TranslationsManager } from "@/applications/translations_editor/store/translations";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectTranslationsPath } from "@/lib/xrf_path";

export function TranslationsEditorOpenForm({
  translationsContext: { project, translationsActions } = useManager(TranslationsManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("translations_editor_open");

  const [translationsPath, setTranslationsPath, onSelectTranslationsPath] = usePathState({
    title: "Provide path to equipment_editor dds",
    filters: [{ name: "dds", extensions: ["dds"] }],
    isDisabled: project.isLoading,
  });

  const onOpenTranslationsClicked = useCallback(() => {
    if (translationsPath) {
      translationsActions.open(translationsPath);
    } else {
      log.info("Cannot open translations when have no provided paths:", {
        translationsPath,
      });
    }
  }, [translationsPath]);

  useEffect(() => {
    if (xrfProjectPath) {
      getPathIfExists(getProjectTranslationsPath(xrfProjectPath)).then((translationsPath) => {
        setTranslationsPath(translationsPath);
      });
    }
  }, []);

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
        <Typography>Provide translations details</Typography>
      </Grid>

      <Grid container sx={{ justifyContent: "center", width: "auto", marginBottom: 2 }}>
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            width: "auto",
            marginRight: 1,
            gap: 2,
          }}
        >
          <FilePickerInput
            label={"Translations path"}
            value={translationsPath}
            disabled={project.isLoading}
            onClick={onSelectTranslationsPath}
          />
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button
            disabled={project.isLoading || !translationsPath}
            variant={"contained"}
            onClick={onOpenTranslationsClicked}
          >
            Open
          </Button>
        </Box>
      </Grid>

      {project.error ? (
        <Box>
          <FormHelperText error>{String(project.error)}</FormHelperText>
        </Box>
      ) : null}

      <ApplicationBackButton disabled={false} path={"/translations_editor"} />
    </Box>
  );
}
