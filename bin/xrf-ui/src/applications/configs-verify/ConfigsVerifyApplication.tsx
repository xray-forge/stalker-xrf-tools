import { Alert } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { ConfigsVerifyResult } from "@/applications/configs-verify/components/ConfigsVerifyResult";
import { PathFormRow } from "@/core/components/form/PathFormRow";
import { IPathField, usePathField } from "@/core/components/form/use-path-field";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ENotificationSeverity, TNotify, useNotify } from "@/core/notifications";
import { EApplicationId } from "@/core/routing/application";
import { getProjectConfigsPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { Logger, useLogger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";
import { commands as configsEditorCommands } from "@/lib/xrf/bindings/xrf-app-configs-editor";
import { LtxProjectVerifyResult } from "@/lib/xrf/bindings/xrf-ltx";

export function ConfigsVerifyApplication(): ReactElement {
  const log: Logger = useLogger("configs-verifier");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<LtxProjectVerifyResult>>(null);

  const configs: IPathField = usePathField({
    id: "configs.verify.directory",
    title: "Provide path to xrf configs",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () => (projectService.xrfProjectPath ? getProjectConfigsPath(projectService.xrfProjectPath) : null),
  });

  const onVerify = useCallback(async () => {
    if (!configs.value) {
      return;
    }

    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Verifying:", configs.value);

      const verified: LtxProjectVerifyResult = await configsEditorCommands.verifyConfigsPath(configs.value);

      setResult(verified);

      notify({
        details: String(configs.value),
        severity: verified.errors.length ? ENotificationSeverity.WARNING : ENotificationSeverity.SUCCESS,
        source: EApplicationId.CONFIGS_VERIFY,
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
        source: EApplicationId.CONFIGS_VERIFY,
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
      isSubmitDisabled={!configs.isValid}
      title={"Verify LTX configs"}
      error={error ?? undefined}
      submitLabel={"Verify"}
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
      onSubmit={onVerify}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Configs directory"}
        description={"Directory of LTX files to validate"}
        field={configs}
      />
    </PickerForm>
  );
}
