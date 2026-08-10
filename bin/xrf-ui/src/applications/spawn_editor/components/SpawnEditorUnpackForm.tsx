import { Alert, Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath, getProjectAllSpawnUnpackPath } from "@/lib/xrf-path";

export function SpawnEditorUnpackForm(): ReactElement {
  const log: Logger = useLogger("spawn-unpack");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const [spawnPath, setSpawnPath, onSelectSpawnPath] = usePathState({
    isDisabled: isLoading,
    title: "Select spawn file",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
  });

  const [outputPath, setOutputPath, onSelectOutputPath] = usePathState({
    isDirectory: true,
    isDisabled: isLoading,
    title: "Select output folder",
  });

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

  const onClearSpawnPath = useCallback(() => setSpawnPath(null), [setSpawnPath]);
  const onClearOutputPath = useCallback(() => setOutputPath(null), [setOutputPath]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath).then((path) => setSpawnPath(path));
      getProjectAllSpawnUnpackPath(projectService.xrfProjectPath).then((path) => setOutputPath(path));
    }
  }, [projectService.xrfProjectPath, setSpawnPath, setOutputPath]);

  return (
    <PickerForm
      title={"Select *.spawn file to unpack"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      isLoading={isLoading}
      backPath={"/spawn_editor"}
      backDisabled={isLoading}
      actions={
        <Button disabled={!spawnPath || !outputPath || isLoading} variant={"contained"} onClick={onUnpackClicked}>
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
      <FilePickerInput
        label={"Source"}
        description={"The packed *.spawn file to read"}
        value={spawnPath}
        isDisabled={isLoading}
        isInvalid={Boolean(spawnFileService.spawnFile.error)}
        onSelect={onSelectSpawnPath}
        onClear={onClearSpawnPath}
      />

      <FilePickerInput
        label={"Destination"}
        description={"Directory the unpacked chunks are written to"}
        value={outputPath}
        isDisabled={isLoading}
        isInvalid={Boolean(spawnFileService.spawnFile.error)}
        onSelect={onSelectOutputPath}
        onClear={onClearOutputPath}
      />
    </PickerForm>
  );
}
