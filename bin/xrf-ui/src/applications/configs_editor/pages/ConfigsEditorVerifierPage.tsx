import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Alert, Button, IconButton, InputAdornment, OutlinedInput, Paper } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, useCallback, useEffect, useState } from "react";

import { ConfigsVerifyResult } from "@/applications/configs_editor/components/ConfigsVerifyResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectVerifyResult } from "@/lib/ltx";

export function ConfigsEditorVerifierPage() {
  const log: Logger = useLogger("configs-verifier");

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<ILtxProjectVerifyResult>>(null);
  const [configsPath, setConfigsPath] = useState<Optional<string>>(projectService.xrfConfigsPath);

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
    [isLoading, log]
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
    setConfigsPath(projectService.xrfConfigsPath);
  }, [projectService.xrfConfigsPath]);

  return (
    <PickerForm
      title={"Provide LTX files directory to verify"}
      error={error ?? undefined}
      isLoading={isLoading}
      backPath={"/configs_editor"}
      actions={
        <Button variant={"contained"} fullWidth disabled={isLoading || !configsPath} onClick={onVerifyPathClicked}>
          Verify
        </Button>
      }
      status={
        result ? (
          result.errors.length ? (
            <Alert severity={"error"}>Configs did not pass validation.</Alert>
          ) : (
            <Alert severity={"success"}>Configs passed validation.</Alert>
          )
        ) : null
      }
      result={
        result ? (
          <Paper elevation={4}>
            <ConfigsVerifyResult result={result} />
          </Paper>
        ) : null
      }
    >
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
    </PickerForm>
  );
}
