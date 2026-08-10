import { Alert } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { ArchivesUnpackResult } from "@/applications/archive-editor/components/ArchivesUnpackResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Nullable } from "@/core/types/general";
import { IArchiveUnpackResult } from "@/lib/archive";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath, getProjectArchivesUnpackPath } from "@/lib/xrf-path";

export function ArchivesEditorUnpackerPage(): ReactElement {
  const log: Logger = useLogger("archives-unpacker");

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<IArchiveUnpackResult>>(null);
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
  }, [archivesPath, archivesUnpackPath, log]);

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
      title={"Provide archives to unpack"}
      error={error ?? undefined}
      isLoading={isLoading}
      backPath={"/archives-editor"}
      backDisabled={isLoading}
      submitLabel={"Unpack"}
      isSubmitDisabled={isLoading || !archivesPath || !archivesUnpackPath}
      onSubmit={onUnpackArchivesPathClicked}
      status={result ? <Alert severity={"success"}>Archives unpacked.</Alert> : null}
      result={result ? <ArchivesUnpackResult result={result} /> : null}
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
