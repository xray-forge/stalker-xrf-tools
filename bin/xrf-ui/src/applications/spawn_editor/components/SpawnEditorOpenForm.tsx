import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, IconButton, InputAdornment, OutlinedInput } from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath } from "@/lib/xrf_path";

export function SpawnEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("spawn-open");

  const { spawnFile, resetSpawnFile, openSpawnFile } = useInjection(SpawnFileService);
  const { xrfProjectPath } = useInjection(ProjectService);

  const [isSelecting, setIsSelecting] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);

  const onSelectSpawnFile = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      try {
        setIsSelecting(true);
        resetSpawnFile();

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
    [log, resetSpawnFile]
  );

  const onSelectSpawnFileClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onOpenSpawnFile = useCallback(() => {
    if (spawnPath) {
      openSpawnFile(spawnPath);
    } else {
      log.info("Cannot parse spawn file without path");
    }
  }, [log, openSpawnFile, spawnPath]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectBuiltAllSpawnPath(xrfProjectPath).then((spawnPath) => {
        setSpawnPath(spawnPath);
      });
    }
  }, []);

  return (
    <PickerForm
      title={"Select *.spawn file to open"}
      error={spawnFile.error ? String(spawnFile.error) : undefined}
      isLoading={spawnFile.isLoading}
      backPath={"/spawn_editor"}
      backDisabled={spawnFile.isLoading || isSelecting}
      actions={
        <Button
          variant={"contained"}
          fullWidth
          disabled={!spawnPath || isSelecting || spawnFile.isLoading}
          onClick={onOpenSpawnFile}
        >
          Open
        </Button>
      }
    >
      <OutlinedInput
        size={"small"}
        disabled={isSelecting}
        value={spawnPath ?? ""}
        placeholder={"Spawn file"}
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
