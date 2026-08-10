import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath } from "@/lib/xrf-path";

export function SpawnEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("spawn-open");

  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const [spawnPath, setSpawnPath, onSelectSpawnPath] = usePathState({
    isDisabled: isLoading,
    title: "Select spawn file",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
  });

  const onOpenSpawnFile = useCallback(() => {
    if (spawnPath) {
      spawnFileService.openSpawnFile(spawnPath);
    } else {
      log.info("Cannot parse spawn file without path");
    }
  }, [log, spawnFileService, spawnPath]);

  const onClearSpawnPath = useCallback(() => setSpawnPath(null), [setSpawnPath]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath).then((path) => setSpawnPath(path));
    }
  }, [projectService.xrfProjectPath, setSpawnPath]);

  return (
    <PickerForm
      title={"Select *.spawn file to open"}
      error={spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : undefined}
      isLoading={isLoading}
      backPath={"/spawn_editor"}
      backDisabled={isLoading}
      actions={
        <Button variant={"contained"} disabled={!spawnPath || isLoading} onClick={onOpenSpawnFile}>
          Open
        </Button>
      }
    >
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(spawnFileService.spawnFile.error)}
        label={"Spawn file"}
        description={"The *.spawn file to read into the editor"}
        value={spawnPath}
        onSelect={onSelectSpawnPath}
        onClear={onClearSpawnPath}
      />
    </PickerForm>
  );
}
