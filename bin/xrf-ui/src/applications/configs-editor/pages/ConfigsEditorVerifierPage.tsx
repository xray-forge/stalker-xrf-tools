import { Alert } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { ConfigsVerifyResult } from "@/applications/configs-editor/components/ConfigsVerifyResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { EApplicationToolId } from "@/core/components/shell/application-tools";
import { ProjectService } from "@/core/store/project";
import { Nullable } from "@/core/types/general";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ILtxProjectVerifyResult } from "@/lib/ltx";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";
import { getProjectConfigsPath } from "@/lib/xrf-path";

export function ConfigsEditorVerifierPage(): ReactElement {
  const log: Logger = useLogger("configs-verifier");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<ILtxProjectVerifyResult>>(null);

  const configs: IPathField = usePathField({
    id: "configs.verify.directory",
    title: "Provide path to xrf configs",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () => (projectService.xrfProjectPath ? getProjectConfigsPath(projectService.xrfProjectPath) : null),
  });

  const onVerify = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Verifying:", configs.value);

      const verified: ILtxProjectVerifyResult = await invoke(EConfigsEditorCommand.VERIFY_CONFIGS_PATH, {
        path: configs.value,
      });

      setResult(verified);

      notify({
        details: String(configs.value),
        severity: verified.errors.length ? ENotificationSeverity.WARNING : ENotificationSeverity.SUCCESS,
        source: EApplicationToolId.CONFIGS,
        title: verified.errors.length
          ? `Configs did not pass validation: ${verified.errors.length} problem(s)`
          : "Configs passed validation",
      });
    } catch (caught: unknown) {
      log.error("Verify error:", caught);
      setError(String(caught));

      notify({
        details: `${configs.value}\n${String(caught)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationToolId.CONFIGS,
        title: "Could not verify configs",
      });
    } finally {
      setIsLoading(false);
    }
  }, [configs.value, log, notify]);

  // A different directory invalidates whatever the previous run reported.
  useEffect(() => {
    setResult(null);
    setError(null);
  }, [configs.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Verify LTX configs"}
      error={error ?? undefined}
      backPath={"/configs-editor"}
      backDisabled={isLoading}
      submitLabel={"Verify"}
      isSubmitDisabled={!configs.isValid}
      onSubmit={onVerify}
      status={
        result ? (
          result.errors.length ? (
            <Alert severity={"error"}>Configs did not pass validation.</Alert>
          ) : (
            <Alert severity={"success"}>Configs passed validation.</Alert>
          )
        ) : null
      }
      result={result ? <ConfigsVerifyResult result={result} /> : null}
    >
      <PathFormRow
        label={"Configs directory"}
        description={"Directory of LTX files to validate"}
        isDisabled={isLoading}
        field={configs}
      />
    </PickerForm>
  );
}
