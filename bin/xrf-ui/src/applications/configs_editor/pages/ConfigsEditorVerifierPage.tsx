import { Alert, Button, Paper } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { ConfigsVerifyResult } from "@/applications/configs_editor/components/ConfigsVerifyResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectVerifyResult } from "@/lib/ltx";

export function ConfigsEditorVerifierPage(): ReactElement {
  const log: Logger = useLogger("configs-verifier");

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<ILtxProjectVerifyResult>>(null);

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

  const onVerifyPathClicked = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Verifying:", configsPath);

      const verified: ILtxProjectVerifyResult = await invoke(EConfigsEditorCommand.VERIFY_CONFIGS_PATH, {
        path: configsPath,
      });

      log.info("Verified:", configsPath);

      setResult(verified);
    } catch (caught: unknown) {
      log.error("Verify error:", caught);
      setError(String(caught));
    } finally {
      setIsLoading(false);
    }
  }, [configsPath, log]);

  useEffect(() => {
    setConfigsPath(projectService.xrfConfigsPath);
  }, [projectService.xrfConfigsPath, setConfigsPath]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Provide LTX files directory to verify"}
      error={error ?? undefined}
      backPath={"/configs_editor"}
      backDisabled={isLoading}
      actions={
        <Button variant={"contained"} disabled={isLoading || !configsPath} onClick={onVerifyPathClicked}>
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
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(error)}
        label={"Configs directory"}
        description={"Directory of LTX files to validate"}
        value={configsPath}
        onSelect={onSelectConfigsPath}
      />
    </PickerForm>
  );
}
