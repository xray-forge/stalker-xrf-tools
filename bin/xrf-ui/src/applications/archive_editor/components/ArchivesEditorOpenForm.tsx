import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Alert,
  Box,
  Button,
  CircularProgress,
  Grid,
  IconButton,
  InputAdornment,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, useCallback, useEffect, useState } from "react";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath } from "@/lib/xrf_path";

export function ArchivesEditorOpenForm() {
  const { project, openArchivesProject } = useInjection(ArchivesManager);
  const { xrfProjectPath } = useInjection(ProjectManager);

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
    [log, project.isLoading]
  );

  const onSelectArchivesPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectConfigsPath(event),
    [onSelectConfigsPath]
  );

  const onOpenPathClicked = useCallback(async () => {
    if (archivesPath) {
      openArchivesProject(archivesPath);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archivesPath, log, openArchivesProject]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectLinkedGamePath(xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
    }
  }, [xrfProjectPath]);

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
        <Typography>Provide archives to open</Typography>
      </Grid>

      <Grid container sx={{ justifyContent: "center", alignItems: "center", width: "auto", marginBottom: 2 }}>
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            width: "auto",
            marginRight: 1,
            gap: 1,
          }}
        >
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
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button variant={"contained"} disabled={project.isLoading || !archivesPath} onClick={onOpenPathClicked}>
            Open
          </Button>
        </Box>
      </Grid>

      {project.isLoading ? <CircularProgress size={24} /> : null}

      {project.error ? (
        <Box sx={{ maxWidth: 540 }}>
          <Alert severity={"error"}>{project.error.message}</Alert>
        </Box>
      ) : null}

      <ApplicationBackButton disabled={project.isLoading} path={"/archives_editor"} />
    </Box>
  );
}
