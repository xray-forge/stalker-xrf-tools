import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Alert,
  Button,
  CircularProgress,
  Grid,
  IconButton,
  InputAdornment,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useManager } from "dreamstate";
import { MouseEvent, useCallback, useEffect, useState } from "react";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath } from "@/lib/xrf_path";

export function ArchivesEditorOpenForm({
  archivesContext: { archiveActions, project } = useManager(ArchivesManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}) {
  const log: Logger = useLogger("archives-editor");
  const [archivesPath, setArchivesPath] = useState<Optional<string>>(null);

  const onSelectConfigsPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      if (project.isLoading) {
        return;
      }

      event.stopPropagation();
      event.preventDefault();

      const newXrfConfigsPath: Optional<string> = (await open({
        title: "Provide path to packed archives",
        directory: true,
      })) as Optional<string>;

      if (newXrfConfigsPath) {
        log.info("Selected new archives path:", newXrfConfigsPath);

        setArchivesPath(newXrfConfigsPath);
      }
    },
    [project.isLoading]
  );

  const onSelectArchivesPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectConfigsPath(event),
    [onSelectConfigsPath]
  );

  const onOpenPathClicked = useCallback(async () => {
    if (archivesPath) {
      archiveActions.open(archivesPath);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archivesPath, log]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectLinkedGamePath(xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
    }
  }, [xrfProjectPath]);

  return (
    <Grid
      justifyContent={"safe center"}
      alignItems={"safe center"}
      direction={"column"}
      flexWrap={"nowrap"}
      container={true}
      width={"100%"}
      height={"100%"}
      padding={4}
    >
      <Grid direction={"row"} justifyContent={"center"} flexShrink={0} marginBottom={2} container item>
        <Typography>Provide archives to open</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} alignItems={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} gap={1} container item>
          <OutlinedInput
            size={"small"}
            disabled={project.isLoading}
            value={archivesPath || ""}
            placeholder={"Source"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectConfigsPath}>
                <IconButton disabled={project.isLoading} edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectArchivesPathClicked}
          />
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button variant={"contained"} disabled={project.isLoading || !archivesPath} onClick={onOpenPathClicked}>
            Open
          </Button>
        </Grid>
      </Grid>

      {project.isLoading ? <CircularProgress size={24} /> : null}

      {project.error ? (
        <Grid maxWidth={540}>
          <Alert severity={"error"}>{project.error.message}</Alert>
        </Grid>
      ) : null}

      <ApplicationBackButton disabled={project.isLoading} path={"/archives_editor"} />
    </Grid>
  );
}
