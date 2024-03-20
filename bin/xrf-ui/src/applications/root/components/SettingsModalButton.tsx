import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as SettingsIcon } from "@mui/icons-material/Settings";
import {
  Dialog,
  FormControl,
  Grid,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/api/dialog";
import { exists } from "@tauri-apps/api/fs";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useState } from "react";

import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getProjectConfigsPath } from "@/lib/xrf_path";

export function SettingsModalButton({
  projectContext: { projectActions, xrfProjectPath, xrfConfigsPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("settings-modal");
  const [isModalOpen, setModalOpen] = useState(false);

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
    [onSelectProjectPath]
  );

  return (
    <>
      <IconButton onClick={() => setModalOpen(true)}>
        <SettingsIcon />
      </IconButton>

      <Dialog open={isModalOpen} onClose={() => setModalOpen(false)}>
        <Grid padding={2} direction={"column"} container>
          <Typography variant={"h6"} component={"h2"}>
            Settings
          </Typography>

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
        </Grid>
      </Dialog>
    </>
  );
}
