import { Alert, Button, Checkbox, FormControlLabel, Paper } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ChangeEvent, useCallback, useEffect, useState } from "react";

import { ConfigsFormatResult } from "@/applications/configs_editor/components/ConfigsFormatResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectFormatResult } from "@/lib/ltx";

export function ConfigsEditorFormatterPage() {
  const log: Logger = useLogger("configs-formatter");

  const projectService: ProjectService = useInjection(ProjectService);

  const [isCheck, setIsCheck] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<ILtxProjectFormatResult>>(null);
  const [configsPath, setConfigsPath, selectConfigsPath] = usePathState({
    isDirectory: true,
    isDisabled: isLoading,
    title: "Provide path to xrf configs",
  });

  // Picking a different directory invalidates whatever the previous run reported.
  const onSelectConfigsPath = useCallback(async () => {
    setError(null);
    setResult(null);

    await selectConfigsPath();
  }, [selectConfigsPath]);

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
    setConfigsPath(projectService.xrfConfigsPath);
  }, [projectService.xrfConfigsPath, setConfigsPath]);

  return (
    <PickerForm
      title={`Provide LTX files directory to ${isCheck ? "check format" : "format"}`}
      error={error ?? undefined}
      isLoading={isLoading}
      backPath={"/configs_editor"}
      actions={
        <Button variant={"contained"} fullWidth disabled={isLoading || !configsPath} onClick={onFormatPathClicked}>
          Format
        </Button>
      }
      status={
        result ? (
          result.toFormat.length ? (
            isCheck ? (
              <Alert severity={"error"}>There are files with invalid formatting.</Alert>
            ) : (
              <Alert severity={"warning"}>Formatted {result.toFormat.length} file(s).</Alert>
            )
          ) : (
            <Alert severity={"success"}>All files are in correct format.</Alert>
          )
        ) : null
      }
      result={
        result ? (
          <Paper elevation={4}>
            <ConfigsFormatResult isCheck={isCheck} result={result} />
          </Paper>
        ) : null
      }
    >
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(error)}
        label={"Configs directory"}
        description={"Directory of LTX files to format"}
        value={configsPath}
        onSelect={onSelectConfigsPath}
      />

      <FormControlLabel
        control={<Checkbox disabled={isLoading} checked={isCheck} onChange={onCheckModeChange} />}
        label={"Check mode (readonly)"}
      />
    </PickerForm>
  );
}
