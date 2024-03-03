import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Button,
  ButtonGroup,
  Card,
  FormHelperText,
  Grid,
  IconButton,
  InputAdornment,
  OutlinedInput,
  Stack,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/api/dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, RefObject, useCallback, useRef, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function SpawnSelectionForm({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
}): ReactElement {
  const log: Logger = useLogger("spawn-select");
  const navigate: NavigateFunction = useNavigate();
  const inputRef: RefObject<HTMLInputElement> = useRef(null);

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

  const onParseSpawnFile = useCallback(() => {
    if (spawnPath) {
      spawnActions.openSpawnFile(spawnPath);
    } else {
      log.info("Cannot parse spawn file without path");
    }
  }, [spawnPath]);

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
    >
      <Grid direction={"row"} justifyContent={"center"} marginBottom={2} container item>
        <Typography>Select *.spawn file</Typography>
      </Grid>

      <Stack direction={"row"} spacing={2} minWidth={350}>
        <OutlinedInput
          ref={inputRef}
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
          onClick={onParseSpawnFile}
        >
          Open
        </Button>
      </Stack>

      {spawnFile.error ? (
        <Grid>
          <FormHelperText error>{String(spawnFile.error)}</FormHelperText>
        </Grid>
      ) : null}

      <Card sx={{ minWidth: 200, marginTop: 2 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button
              disabled={spawnFile.isLoading || isSelecting}
              onClick={() => navigate("/spawn_editor", { replace: true })}
            >
              Back
            </Button>
          </ButtonGroup>
        </Grid>
      </Card>
    </Grid>
  );
}
