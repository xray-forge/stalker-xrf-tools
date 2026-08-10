import { Alert } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectUnpackedAllSpawnPath, getProjectAllSpawnRepackPath } from "@/lib/xrf-path";

export function SpawnEditorPackForm(): ReactElement {
  const log: Logger = useLogger("spawn-pack");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const source: IPathField = usePathField({
    id: "spawn.pack.source",
    title: "Select unpacked spawn folder",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectUnpackedAllSpawnPath(projectService.xrfProjectPath) : null,
  });

  const destination: IPathField = usePathField({
    id: "spawn.pack.destination",
    title: "Select spawn file output",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isSave: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getProjectAllSpawnRepackPath(projectService.xrfProjectPath) : null,
  });

  const onPack = useCallback(async () => {
    log.info("Packing path:", source.value, destination.value);

    setIsFinishedSuccessfully(false);

    if (!source.value || !destination.value) {
      return log.error("Cannot pack file, expected correct paths");
    }

    try {
      await spawnFileService.importSpawnFile(source.value);
      await spawnFileService.saveSpawnFile(destination.value);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to pack file:", error);
    } finally {
      await spawnFileService.closeSpawnFile();
    }
  }, [log, source.value, destination.value, spawnFileService]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Pack spawn file"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      backPath={"/spawn_editor"}
      backDisabled={isLoading}
      submitLabel={"Pack"}
      isSubmitDisabled={!source.isValid || !destination.isValid}
      onSubmit={onPack}
      status={
        isFinishedSuccessfully ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully packed spawn to {destination.value}
          </Alert>
        ) : null
      }
    >
      <PathFormRow
        label={"Source"}
        description={"Directory holding the unpacked spawn chunks"}
        isDisabled={isLoading}
        field={source}
      />

      <PathFormRow
        label={"Output spawn"}
        description={"Where the packed *.spawn file is written"}
        isDisabled={isLoading}
        field={destination}
      />
    </PickerForm>
  );
}
