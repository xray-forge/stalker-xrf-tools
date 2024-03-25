import { default as FolderIcon } from "@mui/icons-material/Folder";
import { FormControl, Grid, IconButton, InputAdornment, InputLabel, OutlinedInput, Typography } from "@mui/material";
import { open } from "@tauri-apps/api/dialog";
import { exists } from "@tauri-apps/api/fs";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, ReactNode, useCallback } from "react";

import { IProjectContext, ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getProjectConfigsPath } from "@/lib/xrf_path";

export interface ISettingsFormProps {
  title?: ReactNode;
  isWithProjectForm?: boolean;
  isWithConfigsForm?: boolean;
  padding?: number | string;
  projectContext?: IProjectContext;
}

export function SettingsForm({
  title,
  isWithProjectForm = true,
  isWithConfigsForm = true,
  padding = 2,
  projectContext: { projectActions, xrfProjectPath, xrfConfigsPath } = useManager(ProjectManager),
}: ISettingsFormProps): ReactElement {
  const log: Logger = useLogger("settings-modal");

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

      // Try to auto-guess configs folder from xrf directory.
      if (!xrfConfigsPath) {
        const newXrfConfigsPath: string = await getProjectConfigsPath(newXrfProjectPath);

        if (await exists(newXrfConfigsPath)) {
          log.info("Automatically selected new configs path:", newXrfConfigsPath);
          projectActions.setXrfConfigsPath(newXrfConfigsPath);
        }
      }
    }
  }, []);

  const onSelectProjectPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectProjectPath(event),
    [onSelectProjectPath]
  );

  const onSelectConfigsPath = useCallback(async (event: MouseEvent<HTMLInputElement>) => {
    event.stopPropagation();
    event.preventDefault();

    const newXrfConfigsPath: Optional<string> = (await open({
      title: "Provide path to xrf configs",
      directory: true,
    })) as Optional<string>;

    if (newXrfConfigsPath) {
      log.info("Selected new configs path:", newXrfConfigsPath);

      projectActions.setXrfConfigsPath(newXrfConfigsPath);
    }
  }, []);

  const onSelectConfigsPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectConfigsPath(event),
    [onSelectConfigsPath]
  );

  return (
    <Grid direction={"column"} padding={padding} container>
      {title ? (
        <Typography variant={"h6"} component={"h2"}>
          {title}
        </Typography>
      ) : null}

      {isWithProjectForm ? (
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
      ) : null}

      {isWithConfigsForm ? (
        <FormControl sx={{ m: "8px 0" }} size={"small"} variant={"outlined"}>
          <InputLabel size={"small"}>Configs</InputLabel>
          <OutlinedInput
            size={"small"}
            type={"text"}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectConfigsPath}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            label={"Configs"}
            value={xrfConfigsPath || ""}
            readOnly
            onClick={onSelectConfigsPathClicked}
          />
        </FormControl>
      ) : null}
    </Grid>
  );
}
