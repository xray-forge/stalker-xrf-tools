import { Alert } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath, getProjectAllSpawnUnpackPath } from "@/lib/xrf-path";

export function SpawnEditorUnpackForm(): ReactElement {
  const log: Logger = useLogger("spawn-unpack");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const source: IPathField = usePathField({
    id: "spawn.unpack.source",
    title: "Select spawn file",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath) : null,
  });

  const destination: IPathField = usePathField({
    id: "spawn.unpack.destination",
    title: "Select output folder",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getProjectAllSpawnUnpackPath(projectService.xrfProjectPath) : null,
  });

  const onUnpack = useCallback(async () => {
    log.info("Unpacking file:", source.value, destination.value);

    setIsFinishedSuccessfully(false);

    if (!source.value || !destination.value) {
      return log.error("Cannot unpack file, expected correct paths");
    }

    try {
      await spawnFileService.openSpawnFile(source.value);
      await spawnFileService.exportSpawnFile(destination.value);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to unpack file:", error);
    } finally {
      await spawnFileService.closeSpawnFile();
    }
  }, [log, source.value, destination.value, spawnFileService]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Unpack spawn file"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      backPath={"/spawn_editor"}
      backDisabled={isLoading}
      submitLabel={"Unpack"}
      isSubmitDisabled={!source.isValid || !destination.isValid}
      status={
        isFinishedSuccessfully ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully unpacked spawn to {destination.value}
          </Alert>
        ) : null
      }
      onSubmit={onUnpack}
    >
      <PathFormRow
        label={"Source"}
        description={"The packed *.spawn file to read"}
        isDisabled={isLoading}
        field={source}
      />

      <PathFormRow
        label={"Destination"}
        description={"Directory the unpacked chunks are written to"}
        isDisabled={isLoading}
        field={destination}
      />
    </PickerForm>
  );
}
