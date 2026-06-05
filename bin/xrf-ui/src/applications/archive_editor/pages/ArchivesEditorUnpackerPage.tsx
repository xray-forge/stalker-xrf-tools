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
  Paper,
  Typography,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useManager } from "dreamstate";
import { MouseEvent, useCallback, useEffect, useState } from "react";

import { ArchivesUnpackResult } from "@/applications/archive_editor/components/ArchivesUnpackResult";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { IArchiveUnpackResult } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath, getProjectArchivesUnpackPath } from "@/lib/xrf_path";

export function ArchivesEditorUnpackerPage({ projectContext: { xrfProjectPath } = useManager(ProjectManager) }) {
  const log: Logger = useLogger("archives-unpacker");

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<IArchiveUnpackResult>>(null);
  const [archivesPath, setArchivesPath] = useState<Optional<string>>(null);
  const [archivesUnpackPath, setArchivesUnpackPath] = useState<Optional<string>>(null);

  const onSelectArchivesPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      if (isLoading) {
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

        setError(null);
        setResult(null);
        setArchivesPath(newXrfConfigsPath);
      }
    },
    [isLoading]
  );

  const onSelectArchivesPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectArchivesPath(event),
    [onSelectArchivesPath]
  );

  const onUnpackArchivesPathClicked = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Unpacking:", archivesPath);

      const result: IArchiveUnpackResult = await invoke(EArchivesEditorCommand.UNPACK_ARCHIVES_PATH, {
        from: archivesPath,
        destination: archivesUnpackPath,
      });

      log.info("Unpacked:", archivesPath);

      setResult(result);
    } catch (error: unknown) {
      log.error("Unpack error:", error);
      setError(String(error));
    } finally {
      setIsLoading(false);
    }
  }, [archivesPath, log]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectLinkedGamePath(xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
      getProjectArchivesUnpackPath(xrfProjectPath).then((unpackPath) => setArchivesUnpackPath(unpackPath));
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
        <Typography>Provide archives to unpack</Typography>
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
            disabled={isLoading}
            value={archivesPath || ""}
            placeholder={"Source"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectArchivesPath}>
                <IconButton disabled={isLoading} edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectArchivesPathClicked}
          />

          <OutlinedInput
            size={"small"}
            disabled={isLoading}
            value={archivesUnpackPath || ""}
            placeholder={"Output"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectArchivesPath}>
                <IconButton disabled={isLoading} edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectArchivesPathClicked}
          />
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button variant={"contained"} disabled={isLoading || !archivesPath} onClick={onUnpackArchivesPathClicked}>
            Unpack
          </Button>
        </Box>
      </Grid>

      {isLoading ? <CircularProgress size={24} /> : null}

      {result ? (
        <Box>
          <Alert severity={"success"}>Archives unpacked.</Alert>
        </Box>
      ) : null}

      {error ? (
        <Box sx={{ maxWidth: 540 }}>
          <Alert severity={"error"}>{error}</Alert>
        </Box>
      ) : null}

      <ApplicationBackButton disabled={isLoading} path={"/archives_editor"} />

      {result ? (
        <Paper elevation={4}>
          <ArchivesUnpackResult result={result} />
        </Paper>
      ) : null}
    </Box>
  );
}
