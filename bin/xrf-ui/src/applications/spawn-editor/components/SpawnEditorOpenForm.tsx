import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath } from "@/lib/xrf-path";

export function SpawnEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("spawn-open");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const spawn: IPathField = usePathField({
    id: "spawn.open.file",
    title: "Select spawn file",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath) : null,
  });

  const onOpen = useCallback(() => {
    if (spawn.value) {
      spawnFileService.openSpawnFile(spawn.value);
    } else {
      log.info("Cannot parse spawn file without path");
    }
  }, [log, spawnFileService, spawn.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open spawn file"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      backPath={"/spawn-editor"}
      backDisabled={isLoading}
      submitLabel={"Open"}
      isSubmitDisabled={!spawn.isValid}
      onSubmit={onOpen}
    >
      <PathFormRow
        label={"Spawn file"}
        description={"The *.spawn file to read into the editor"}
        isDisabled={isLoading}
        field={spawn}
      />
    </PickerForm>
  );
}
