import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Box,
  Button,
  FormControl,
  Grid,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback } from "react";

import { ExportsManager } from "@/applications/exports_editor/store/exports";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function ExportsOpenForm({
  exportsContext: { exportsActions, declarations } = useManager(ExportsManager),
  projectContext: { projectActions, xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("exports-open");

  const onSelectProjectPath = useCallback(async (event: MouseEvent<HTMLInputElement>) => {
    event.stopPropagation();
    event.preventDefault();

    const newXrfProjectPath: Optional<string> = (await open({
      title: "Provide path to xrf project",
      directory: true,
    })) as Optional<string>;

    if (newXrfProjectPath) {
      log.info("Selected new project path:", newXrfProjectPath);
      projectActions.setXrfProjectPath(newXrfProjectPath);
    }
  }, []);

  const onSelectProjectPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectProjectPath(event),
    [onSelectProjectPath]
  );

  const onOpenExportsClicked = useCallback(() => {
    if (xrfProjectPath) {
      exportsActions.open(xrfProjectPath);
    } else {
      log.info("Cannot open exports when have no project path");
    }
  }, [exportsActions, xrfProjectPath, exportsActions]);

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
      <Grid container sx={{ justifyContent: "center", marginBottom: 2 }}>
        <Typography>Provide paths to ltx project</Typography>
      </Grid>

      <Grid container sx={{ gap: 1, alignItems: "center", justifyContent: "center" }}>
        <FormControl sx={{ m: "8px 0" }} size={"small"} variant={"outlined"}>
          <InputLabel size={"small"}>Project</InputLabel>
          <OutlinedInput
            size={"small"}
            type={"text"}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectProjectPath}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            label={"Project"}
            value={xrfProjectPath || ""}
            readOnly
            onClick={onSelectProjectPathClicked}
          />
        </FormControl>

        <Button
          disabled={!xrfProjectPath || declarations.isLoading}
          variant={"contained"}
          onClick={onOpenExportsClicked}
        >
          Open exports
        </Button>
      </Grid>

      <ApplicationBackButton path={"/exports_editor"} />
    </Box>
  );
}
