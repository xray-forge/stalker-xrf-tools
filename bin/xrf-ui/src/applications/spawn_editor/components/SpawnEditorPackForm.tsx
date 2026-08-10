import { Alert, Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectUnpackedAllSpawnPath, getProjectAllSpawnRepackPath } from "@/lib/xrf-path";

export function SpawnEditorPackForm(): ReactElement {
  const log: Logger = useLogger("spawn-pack");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const [inputPath, setInputPath, onSelectInputPath] = usePathState({
    isDirectory: true,
    isDisabled: isLoading,
    title: "Select unpacked spawn folder",
  });

  const [spawnPath, setSpawnPath, onSelectSpawnPath] = usePathState({
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isDisabled: isLoading,
    isSave: true,
    title: "Select spawn file output",
  });

  const onPackClicked = useCallback(async () => {
    log.info("Packing path:", inputPath, spawnPath);

    setIsFinishedSuccessfully(false);

    if (!spawnPath || !inputPath) {
      return log.error("Cannot pack file, expected correct paths:", spawnPath, inputPath);
    }

    try {
      await spawnFileService.importSpawnFile(inputPath);
      await spawnFileService.saveSpawnFile(spawnPath);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to pack file:", error);
    } finally {
      await spawnFileService.closeSpawnFile();
    }
  }, [log, inputPath, spawnPath, spawnFileService]);

  const onClearInputPath = useCallback(() => setInputPath(null), [setInputPath]);
  const onClearSpawnPath = useCallback(() => setSpawnPath(null), [setSpawnPath]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectUnpackedAllSpawnPath(projectService.xrfProjectPath).then((path) => setInputPath(path));
      getProjectAllSpawnRepackPath(projectService.xrfProjectPath).then((path) => setSpawnPath(path));
    }
  }, [projectService.xrfProjectPath, setInputPath, setSpawnPath]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Select *.spawn file to pack"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      backPath={"/spawn_editor"}
      backDisabled={isLoading}
      actions={
        <Button disabled={!spawnPath || !inputPath || isLoading} variant={"contained"} onClick={onPackClicked}>
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
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(spawnFileService.spawnFile.error)}
        label={"Source"}
        description={"Directory holding the unpacked spawn chunks"}
        value={inputPath}
        onSelect={onSelectInputPath}
        onClear={onClearInputPath}
      />

      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(spawnFileService.spawnFile.error)}
        label={"Output spawn"}
        description={"Where the packed *.spawn file is written"}
        value={spawnPath}
        onSelect={onSelectSpawnPath}
        onClear={onClearSpawnPath}
      />
    </PickerForm>
  );
}
