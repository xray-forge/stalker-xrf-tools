import { Alert } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { ArchivesUnpackResult } from "@/applications/archives-unpack/components/ArchivesUnpackResult";
import { commands as archivesEditorCommands } from "@/core/bindings/xrf-app-archives-editor";
import { ArchiveUnpackResult } from "@/core/bindings/xrf-archive";
import { FilePickerInput } from "@/core/components/form/file-picker/FilePickerInput";
import { usePathState } from "@/core/components/form/file-picker/use-path-state";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ENotificationSeverity, TNotify, useNotify } from "@/core/notifications";
import { EApplicationId } from "@/core/routing/application";
import { getExistingProjectLinkedGamePath, getProjectArchivesUnpackPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { Logger, useLogger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

export function ArchivesUnpackApplication(): ReactElement {
  const log: Logger = useLogger("archives-unpacker");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<ArchiveUnpackResult>>(null);

  const [archivesPath, setArchivesPath, selectArchivesPath] = usePathState({
    isDirectory: true,
    isDisabled: isLoading,
    title: "Provide path to packed archives",
  });

  const [archivesUnpackPath, setArchivesUnpackPath, selectArchivesUnpackPath] = usePathState({
    isDirectory: true,
    isDisabled: isLoading,
    title: "Provide output directory to unpack into",
  });

  // Picking different paths invalidates whatever the previous run reported.
  const onSelectArchivesPath = useCallback(async () => {
    setError(null);
    setResult(null);

    await selectArchivesPath();
  }, [selectArchivesPath]);

  const onSelectArchivesUnpackPath = useCallback(async () => {
    setError(null);
    setResult(null);

    await selectArchivesUnpackPath();
  }, [selectArchivesUnpackPath]);

  const onUnpackArchivesPathClicked = useCallback(async () => {
    if (!archivesPath || !archivesUnpackPath) {
      return;
    }

    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Unpacking:", archivesPath);

      const result: ArchiveUnpackResult = await archivesEditorCommands.unpackArchivesPath(
        archivesPath,
        archivesUnpackPath
      );

      log.info("Unpacked:", archivesPath);

      setResult(result);

      notify({
        details: `${archivesPath}\n${archivesUnpackPath}`,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.ARCHIVES_UNPACK,
        title: "Unpacked archives",
      });
    } catch (error: unknown) {
      log.error("Unpack error:", error);
      setError(String(error));

      notify({
        details: `${archivesPath}\n${String(error)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.ARCHIVES_UNPACK,
        title: "Could not unpack archives",
      });
    } finally {
      setIsLoading(false);
    }
  }, [archivesPath, archivesUnpackPath, log, notify]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectLinkedGamePath(projectService.xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
      getProjectArchivesUnpackPath(projectService.xrfProjectPath).then((unpackPath) =>
        setArchivesUnpackPath(unpackPath)
      );
    }
  }, [projectService.xrfProjectPath, setArchivesPath, setArchivesUnpackPath]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={isLoading || !archivesPath || !archivesUnpackPath}
      title={"Provide archives to unpack"}
      error={error ?? undefined}
      submitLabel={"Unpack"}
      status={result ? <Alert severity={"success"}>Archives unpacked.</Alert> : null}
      result={result ? <ArchivesUnpackResult result={result} /> : null}
      onSubmit={onUnpackArchivesPathClicked}
    >
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(error)}
        label={"Source"}
        description={"Directory holding the packed game archives"}
        value={archivesPath}
        onSelect={onSelectArchivesPath}
      />

      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(error)}
        label={"Output"}
        description={"Directory the archives are unpacked into"}
        value={archivesUnpackPath}
        onSelect={onSelectArchivesUnpackPath}
      />
    </PickerForm>
  );
}
