import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Alert, Button, IconButton, InputAdornment, OutlinedInput } from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath, getProjectAllSpawnUnpackPath } from "@/lib/xrf_path";

export function SpawnEditorUnpackForm(): ReactElement {
  const log: Logger = useLogger("spawn-unpack");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [isSelecting, setIsSelecting] = useState(false);
  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);
  const [outputPath, setOutputPath] = useState<Optional<string>>(null);

  const onSelectSpawnFile = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFileService.spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

        const selected: Optional<string> = (await open({
          title: "Select spawn file",
          filters: [{ name: "spawn", extensions: ["spawn"] }],
        })) as Optional<string>;

        setSpawnPath(selected);

        log.info("Selected new file:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [log, spawnFileService.spawnFile.isLoading]
  );

  const onSelectSpawnFileClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onSelectOutput = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFileService.spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

        const selected: Optional<string> = (await open({
          title: "Select output folder",
          directory: true,
        })) as Optional<string>;

        setOutputPath(selected);

        log.info("Selected output:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [log, spawnFileService.spawnFile.isLoading]
  );

  const onSelectOutputClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onUnpackClicked = useCallback(async () => {
    log.info("Unpacking file:", spawnPath, outputPath);

    setIsFinishedSuccessfully(false);

    if (!spawnPath || !outputPath) {
      return log.error("Cannot unpack file, expected correct paths:", spawnPath, outputPath);
    }

    try {
      await spawnFileService.openSpawnFile(spawnPath);
      await spawnFileService.exportSpawnFile(outputPath);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to unpack file:", error);
    } finally {
      await spawnFileService.closeSpawnFile();
    }
  }, [log, spawnPath, outputPath, spawnFileService]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath).then((spawnPath) => {
        setSpawnPath(spawnPath);
      });

      getProjectAllSpawnUnpackPath(projectService.xrfProjectPath).then((outputPath) => setOutputPath(outputPath));
    }
  }, []);

  return (
    <PickerForm
      title={"Select *.spawn file to unpack"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      isLoading={spawnFileService.spawnFile.isLoading}
      backPath={"/spawn_editor"}
      backDisabled={spawnFileService.spawnFile.isLoading || isSelecting}
      actions={
        <Button
          fullWidth
          disabled={!spawnPath || !outputPath || isSelecting || spawnFileService.spawnFile.isLoading}
          variant={"contained"}
          onClick={onUnpackClicked}
        >
          Unpack
        </Button>
      }
      status={
        isFinishedSuccessfully ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully unpacked spawn to {outputPath}
          </Alert>
        ) : null
      }
    >
      <OutlinedInput
        size={"small"}
        disabled={isSelecting || spawnFileService.spawnFile.isLoading}
        value={spawnPath ?? ""}
        placeholder={"Source"}
        readOnly={true}
        error={Boolean(spawnFileService.spawnFile.error)}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectSpawnFile}>
            <IconButton edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectSpawnFileClicked}
      />

      <OutlinedInput
        size={"small"}
        disabled={isSelecting || spawnFileService.spawnFile.isLoading}
        value={outputPath ?? ""}
        placeholder={"Destination"}
        readOnly={true}
        error={Boolean(spawnFileService.spawnFile.error)}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectOutput}>
            <IconButton edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectOutputClicked}
      />
    </PickerForm>
  );
}
