import { Alert, Checkbox, FormControlLabel } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ChangeEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { ConfigsFormatResult } from "@/applications/configs-format/components/ConfigsFormatResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { EApplicationId } from "@/core/router/application";
import { ProjectService } from "@/core/store/project";
import { Nullable } from "@/core/types/general";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { EConfigsEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";
import { LtxProjectFormatResult } from "@/lib/xrf/bindings/xray-ltx";
import { getProjectConfigsPath } from "@/lib/xrf-path";

export function ConfigsFormatApplication(): ReactElement {
  const log: Logger = useLogger("configs-formatter");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isCheck, setIsCheck] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<LtxProjectFormatResult>>(null);

  const configs: IPathField = usePathField({
    id: "configs.format.directory",
    title: "Provide path to xrf configs",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () => (projectService.xrfProjectPath ? getProjectConfigsPath(projectService.xrfProjectPath) : null),
  });

  const onFormat = useCallback(async () => {
    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Performing format command:", isCheck, configs.value);

      const formatted: LtxProjectFormatResult = await invoke(
        isCheck ? EConfigsEditorCommand.CHECK_FORMAT_CONFIGS_PATH : EConfigsEditorCommand.FORMAT_CONFIGS_PATH,
        { path: configs.value }
      );

      setResult(formatted);

      notify({
        details: String(configs.value),
        severity: formatted.toFormat.length
          ? isCheck
            ? ENotificationSeverity.ERROR
            : ENotificationSeverity.WARNING
          : ENotificationSeverity.SUCCESS,
        source: EApplicationId.CONFIGS_FORMAT,
        title: formatted.toFormat.length
          ? isCheck
            ? `${formatted.toFormat.length} file(s) have invalid formatting`
            : `Formatted ${formatted.toFormat.length} file(s)`
          : "All files are in correct format",
      });
    } catch (caught) {
      log.error("Format error:", caught);
      setError(String(caught));

      notify({
        details: `${configs.value}\n${String(caught)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.CONFIGS_FORMAT,
        title: isCheck ? "Could not check formatting" : "Could not format configs",
      });
    } finally {
      setIsLoading(false);
    }
  }, [configs.value, isCheck, log, notify]);

  const onCheckModeChange = useCallback((_: ChangeEvent<HTMLInputElement>, checked: boolean) => {
    setResult(null);
    setError(null);
    setIsCheck(checked);
  }, []);

  useEffect(() => {
    setResult(null);
    setError(null);
  }, [configs.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={!configs.isValid}
      title={isCheck ? "Check LTX formatting" : "Format LTX configs"}
      error={error ?? undefined}
      submitLabel={isCheck ? "Check" : "Format"}
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
      result={result ? <ConfigsFormatResult isCheck={isCheck} result={result} /> : null}
      onSubmit={onFormat}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Configs directory"}
        description={"Directory of LTX files to format"}
        field={configs}
      />

      <FormControlLabel
        control={<Checkbox disabled={isLoading} checked={isCheck} onChange={onCheckModeChange} />}
        label={"Check mode (readonly)"}
      />
    </PickerForm>
  );
}
