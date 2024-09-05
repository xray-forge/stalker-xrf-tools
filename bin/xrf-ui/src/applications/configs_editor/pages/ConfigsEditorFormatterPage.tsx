import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Alert,
  Button,
  Checkbox,
  CircularProgress,
  FormControlLabel,
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
import { ChangeEvent, MouseEvent, useCallback, useEffect, useState } from "react";

import { ConfigsFormatResult } from "@/applications/configs_editor/components/ConfigsFormatResult";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectFormatResult } from "@/lib/ltx";

export function ConfigsEditorFormatterPage({ projectContext: { xrfConfigsPath } = useManager(ProjectManager) }) {
  const log: Logger = useLogger("configs-formatter");

  const [isCheck, setIsCheck] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<ILtxProjectFormatResult>>(null);
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

  const onFormatPathClicked = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Performing format command:", isCheck, configsPath);

      const result: ILtxProjectFormatResult = await invoke(
        isCheck ? EConfigsEditorCommand.CHECK_FORMAT_CONFIGS_PATH : EConfigsEditorCommand.FORMAT_CONFIGS_PATH,
        { path: configsPath }
      );

      log.info("Finished format command:", isCheck, configsPath);

      setResult(result);
    } catch (error) {
      log.error("Format error:", error);
      setError(String(error));
    } finally {
      setIsLoading(false);
    }
  }, [configsPath, isCheck, log]);

  const onCheckModeChange = useCallback((_: ChangeEvent<HTMLInputElement>, checked: boolean) => {
    setResult(null);
    setError(null);
    setIsCheck(checked);
  }, []);

  useEffect(() => {
    setConfigsPath(xrfConfigsPath);
  }, [xrfConfigsPath]);

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
      <Grid direction={"row"} justifyContent={"center"} marginBottom={2} container item>
        <Typography>Provide LTX files directory to {isCheck ? "check format" : "format"}</Typography>
      </Grid>

      <Grid
        direction={"row"}
        justifyContent={"center"}
        alignItems={"flex-start"}
        width={"auto"}
        marginBottom={2}
        container
      >
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} container item>
          <OutlinedInput
            size={"small"}
            disabled={isLoading}
            value={configsPath || ""}
            placeholder={"Configs directory"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectConfigsPath}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectConfigsPathClicked}
          />

          <Grid
            direction={"row"}
            justifyContent={"flex-start"}
            alignItems={"center"}
            width={"auto"}
            marginBottom={2}
            container
          >
            <FormControlLabel
              control={<Checkbox disabled={isLoading} checked={isCheck} onChange={onCheckModeChange} />}
              label={"Check mode (readonly)"}
            />
          </Grid>
        </Grid>

        <Grid position={"relative"} top={2}>
          <Button variant={"contained"} disabled={isLoading || !configsPath} onClick={onFormatPathClicked}>
            Format
          </Button>
        </Grid>
      </Grid>

      {isLoading ? <CircularProgress size={24} /> : null}

      {result ? (
        <Grid>
          {result.toFormat.length ? (
            isCheck ? (
              <Alert severity={"error"}>There are files with invalid formatting.</Alert>
            ) : (
              <Alert severity={"warning"}>Formatted {result.toFormat.length} file(s).</Alert>
            )
          ) : (
            <Alert severity={"success"}>All files are in correct format.</Alert>
          )}
        </Grid>
      ) : null}

      {error ? (
        <Grid maxWidth={540}>
          <Alert severity={"error"}>{error}</Alert>
        </Grid>
      ) : null}

      <ApplicationBackButton path={"/configs_editor"} />

      {result ? (
        <Paper elevation={4}>
          <ConfigsFormatResult isCheck={isCheck} result={result} />
        </Paper>
      ) : null}
    </Grid>
  );
}
