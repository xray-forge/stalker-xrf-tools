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

import { ConfigsVerifyResult } from "@/applications/configs_editor/components/ConfigsVerifyResult";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectVerifyResult } from "@/lib/ltx";

export function ConfigsEditorVerifierPage({ projectContext: { xrfConfigsPath } = useManager(ProjectManager) }) {
  const log: Logger = useLogger("configs-verifier");

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<ILtxProjectVerifyResult>>(null);
  const [configsPath, setConfigsPath] = useState<Optional<string>>(xrfConfigsPath);

  const onSelectConfigsPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      if (isLoading) {
        return;
      }

      event.stopPropagation();
      event.preventDefault();

      const newXrfConfigsPath: Optional<string> = (await open({
        title: "Provide path to xrf configs",
        directory: true,
      })) as Optional<string>;

      if (newXrfConfigsPath) {
        log.info("Selected new configs path:", newXrfConfigsPath);

        setError(null);
        setResult(null);
        setConfigsPath(newXrfConfigsPath);
      }
    },
    [isLoading]
  );

  const onSelectConfigsPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectConfigsPath(event),
    [onSelectConfigsPath]
  );

  const onVerifyPathClicked = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Verifying:", configsPath);

      const result: ILtxProjectVerifyResult = await invoke(EConfigsEditorCommand.VERIFY_CONFIGS_PATH, {
        path: configsPath,
      });

      log.info("Verified:", configsPath);

      setResult(result);
    } catch (error: unknown) {
      log.error("Verify error:", error);
      setError(String(error));
    } finally {
      setIsLoading(false);
    }
  }, [configsPath, log]);

  useEffect(() => {
    setConfigsPath(xrfConfigsPath);
  }, [xrfConfigsPath]);

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
        <Typography>Provide LTX files directory to verify</Typography>
      </Grid>

      <Grid container sx={{ justifyContent: "center", alignItems: "center", width: "auto", marginBottom: 2 }}>
        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto", marginRight: 1 }}>
          <OutlinedInput
            size={"small"}
            disabled={isLoading}
            value={configsPath || ""}
            placeholder={"Configs directory"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectConfigsPath}>
                <IconButton disabled={isLoading} edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectConfigsPathClicked}
          />
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button variant={"contained"} disabled={isLoading || !configsPath} onClick={onVerifyPathClicked}>
            Verify
          </Button>
        </Box>
      </Grid>

      {isLoading ? <CircularProgress size={24} /> : null}

      {result ? (
        <Box>
          {result.errors.length ? (
            <Alert severity={"error"}>Configs did not pass validation.</Alert>
          ) : (
            <Alert severity={"success"}>Configs passed validation.</Alert>
          )}
        </Box>
      ) : null}

      {error ? (
        <Box sx={{ maxWidth: 540 }}>
          <Alert severity={"error"}>{error}</Alert>
        </Box>
      ) : null}

      <ApplicationBackButton path={"/configs_editor"} />

      {result ? (
        <Paper elevation={4}>
          <ConfigsVerifyResult result={result} />
        </Paper>
      ) : null}
    </Box>
  );
}
