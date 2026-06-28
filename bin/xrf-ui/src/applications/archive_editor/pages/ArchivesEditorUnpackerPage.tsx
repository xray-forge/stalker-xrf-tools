import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Alert, Button, IconButton, InputAdornment, OutlinedInput, Paper } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, useCallback, useEffect, useState } from "react";

import { ArchivesUnpackResult } from "@/applications/archive_editor/components/ArchivesUnpackResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { IArchiveUnpackResult } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath, getProjectArchivesUnpackPath } from "@/lib/xrf_path";

export function ArchivesEditorUnpackerPage() {
  const log: Logger = useLogger("archives-unpacker");

  const { xrfProjectPath } = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Optional<string>>(null);
  const [result, setResult] = useState<Optional<IArchiveUnpackResult>>(null);
  const [archivesPath, setArchivesPath] = useState<Optional<string>>(null);
  const [archivesUnpackPath, setArchivesUnpackPath] = useState<Optional<string>>(null);

  const onSelectArchivesPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      if (isLoading) {
        return;
      }

      event.stopPropagation();
      event.preventDefault();

      const newXrfConfigsPath: Optional<string> = (await open({
        title: "Provide path to packed archives",
        directory: true,
      })) as Optional<string>;

      if (newXrfConfigsPath) {
        log.info("Selected new archives path:", newXrfConfigsPath);

        setError(null);
        setResult(null);
        setArchivesPath(newXrfConfigsPath);
      }
    },
    [isLoading, log]
  );

  const onSelectArchivesPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectArchivesPath(event),
    [onSelectArchivesPath]
  );

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
    if (xrfProjectPath) {
      getExistingProjectLinkedGamePath(xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
      getProjectArchivesUnpackPath(xrfProjectPath).then((unpackPath) => setArchivesUnpackPath(unpackPath));
    }
  }, [xrfProjectPath]);

  return (
    <PickerForm
      title={"Provide archives to unpack"}
      error={error ?? undefined}
      isLoading={isLoading}
      backPath={"/archives_editor"}
      backDisabled={isLoading}
      actions={
        <Button
          variant={"contained"}
          fullWidth
          disabled={isLoading || !archivesPath}
          onClick={onUnpackArchivesPathClicked}
        >
          Unpack
        </Button>
      }
      status={result ? <Alert severity={"success"}>Archives unpacked.</Alert> : null}
      result={
        result ? (
          <Paper elevation={4}>
            <ArchivesUnpackResult result={result} />
          </Paper>
        ) : null
      }
    >
      <OutlinedInput
        size={"small"}
        disabled={isLoading}
        value={archivesPath || ""}
        placeholder={"Source"}
        readOnly={true}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectArchivesPath}>
            <IconButton disabled={isLoading} edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectArchivesPathClicked}
      />

      <OutlinedInput
        size={"small"}
        disabled={isLoading}
        value={archivesUnpackPath || ""}
        placeholder={"Output"}
        readOnly={true}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectArchivesPath}>
            <IconButton disabled={isLoading} edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectArchivesPathClicked}
      />
    </PickerForm>
  );
}
