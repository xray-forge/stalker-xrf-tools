import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { PathFormRow } from "@/core/components/form/PathFormRow";
import { IPathField, usePathField } from "@/core/components/form/use-path-field";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { getExistingProjectBuiltAllSpawnPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { SpawnFileService } from "@/core/spawn-file/services";
import { Logger, useLogger } from "@/lib/logging";

export function SpawnEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("spawn-open");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = spawnFileService.header.isLoading;

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
      isSubmitDisabled={!spawn.isValid}
      title={"Open spawn file"}
      error={spawnFileService.header.error ? String(spawnFileService.header.error) : undefined}
      submitLabel={"Open"}
      onSubmit={onOpen}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Spawn file"}
        description={"The *.spawn file to read into the editor"}
        field={spawn}
      />
    </PickerForm>
  );
}
