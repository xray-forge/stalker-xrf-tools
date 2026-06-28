import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Alert, Button, IconButton, InputAdornment, OutlinedInput } from "@mui/material";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectUnpackedAllSpawnPath, getProjectAllSpawnRepackPath } from "@/lib/xrf_path";

export function SpawnEditorPackForm(): ReactElement {
  const log: Logger = useLogger("spawn-pack");

  const { spawnFile, importSpawnFile, saveSpawnFile, closeSpawnFile } = useInjection(SpawnFileService);
  const { xrfProjectPath } = useInjection(ProjectService);

  const [isSelecting, setIsSelecting] = useState(false);
  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);
  const [inputPath, setInputPath] = useState<Optional<string>>(null);

  const onSelectSpawnFile = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

        const selected: Optional<string> = (await save({
          title: "Select spawn file output",
          filters: [{ name: "spawn", extensions: ["spawn"] }],
        })) as Optional<string>;

        setSpawnPath(selected);

        log.info("Selected file output:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [log, spawnFile.isLoading]
  );

  const onSelectSpawnFileClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onSelectInput = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

        const selected: Optional<string> = (await open({
          title: "Select unpacked spawn folder",
          directory: true,
        })) as Optional<string>;

        setInputPath(selected);

        log.info("Selected output:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [log, spawnFile.isLoading]
  );

  const onSelectInputClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectInput(event),
    [onSelectInput]
  );

  const onPackClicked = useCallback(async () => {
    log.info("Packing path:", inputPath, spawnPath);

    setIsFinishedSuccessfully(false);

    if (!spawnPath || !inputPath) {
      return log.error("Cannot pack file, expected correct paths:", spawnPath, inputPath);
    }

    try {
      await importSpawnFile(inputPath);
      await saveSpawnFile(spawnPath);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to pack file:", error);
    } finally {
      await closeSpawnFile();
    }
  }, [log, inputPath, spawnPath, importSpawnFile, saveSpawnFile, closeSpawnFile]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectUnpackedAllSpawnPath(xrfProjectPath).then((inputPath) => {
        setInputPath(inputPath);
      });

      getProjectAllSpawnRepackPath(xrfProjectPath).then((outputPath) => setSpawnPath(outputPath));
    }
  }, []);

  return (
    <PickerForm
      title={"Select *.spawn file to pack"}
      error={spawnFile.error ? String(spawnFile.error) : undefined}
      isLoading={spawnFile.isLoading}
      backPath={"/spawn_editor"}
      backDisabled={spawnFile.isLoading || isSelecting}
      actions={
        <Button
          fullWidth
          disabled={!spawnPath || !inputPath || isSelecting || spawnFile.isLoading}
          variant={"contained"}
          onClick={onPackClicked}
        >
          Pack
        </Button>
      }
      status={
        isFinishedSuccessfully ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully packed spawn to {spawnPath}
          </Alert>
        ) : null
      }
    >
      <OutlinedInput
        size={"small"}
        disabled={isSelecting || spawnFile.isLoading}
        value={inputPath ?? ""}
        placeholder={"Source"}
        readOnly={true}
        error={Boolean(spawnFile.error)}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectInputClicked}>
            <IconButton edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectInput}
      />

      <OutlinedInput
        size={"small"}
        disabled={isSelecting || spawnFile.isLoading}
        value={spawnPath ?? ""}
        placeholder={"Output spawn"}
        readOnly={true}
        error={Boolean(spawnFile.error)}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectSpawnFile}>
            <IconButton edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectSpawnFileClicked}
      />
    </PickerForm>
  );
}
