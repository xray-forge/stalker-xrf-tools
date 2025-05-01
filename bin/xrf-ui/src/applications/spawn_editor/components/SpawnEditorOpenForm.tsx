import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Button,
  CircularProgress,
  FormHelperText,
  Grid,
  IconButton,
  InputAdornment,
  OutlinedInput,
  Stack,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectBuiltAllSpawnPath } from "@/lib/xrf_path";

export function SpawnEditorOpenForm({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("spawn-open");

  const [isSelecting, setIsSelecting] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);

  const onSelectSpawnFile = useCallback(async (event: MouseEvent<HTMLInputElement>) => {
    event.stopPropagation();
    event.preventDefault();

    try {
      setIsSelecting(true);
      spawnActions.resetSpawnFile();

      const selected: Optional<string> = (await open({
        title: "Select spawn file",
        filters: [{ name: "spawn", extensions: ["spawn"] }],
      })) as Optional<string>;

      setSpawnPath(selected);

      log.info("Selected new file:", selected);
    } finally {
      setIsSelecting(false);
    }
  }, []);

  const onSelectSpawnFileClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onOpenSpawnFile = useCallback(() => {
    if (spawnPath) {
      spawnActions.openSpawnFile(spawnPath);
    } else {
      log.info("Cannot parse spawn file without path");
    }
  }, [spawnPath]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectBuiltAllSpawnPath(xrfProjectPath).then((spawnPath) => {
        setSpawnPath(spawnPath);
      });
    }
  }, []);

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
    >
      <Grid direction={"row"} justifyContent={"center"} marginBottom={2} container>
        <Typography>Select *.spawn file to open</Typography>
      </Grid>

      <Stack direction={"row"} spacing={1} marginBottom={2} minWidth={350}>
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

        <Button
          variant={"contained"}
          disabled={!spawnPath || isSelecting || spawnFile.isLoading}
          onClick={onOpenSpawnFile}
        >
          Open
        </Button>
      </Stack>

      {spawnFile.error ? (
        <Grid>
          <FormHelperText error>{String(spawnFile.error)}</FormHelperText>
        </Grid>
      ) : null}

      {spawnFile.isLoading ? <CircularProgress size={24} /> : null}

      <ApplicationBackButton disabled={spawnFile.isLoading || isSelecting} path={"/spawn_editor"} />
    </Grid>
  );
}
