import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Alert,
  Button,
  CircularProgress,
  FormHelperText,
  Grid,
  IconButton,
  InputAdornment,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/api/dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, RefObject, useCallback, useRef, useState } from "react";

import { SpawnBackButton } from "@/applications/spawn_editor/components/SpawnBackButton";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function SpawnEditorUnpackForm({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
}): ReactElement {
  const log: Logger = useLogger("spawn-unpack");
  const inputRef: RefObject<HTMLInputElement> = useRef(null);

  const [isSelecting, setIsSelecting] = useState(false);
  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);
  const [outputPath, setOutputPath] = useState<Optional<string>>(null);

  const onSelectSpawnFile = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

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
    [spawnFile]
  );

  const onSelectSpawnFileClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onSelectOutput = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      if (spawnFile.isLoading) {
        return;
      }

      try {
        setIsFinishedSuccessfully(false);
        setIsSelecting(true);

        const selected: Optional<string> = (await open({
          title: "Select output folder",
          directory: true,
        })) as Optional<string>;

        setOutputPath(selected);

        log.info("Selected output:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [spawnFile]
  );

  const onSelectOutputClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectSpawnFile(event),
    [onSelectSpawnFile]
  );

  const onUnpackClicked = useCallback(async () => {
    log.info("Unpacking file:", spawnPath, outputPath);

    setIsFinishedSuccessfully(false);

    if (!spawnPath || !outputPath) {
      return log.error("Cannot unpack file, expected correct paths:", spawnPath, outputPath);
    }

    try {
      await spawnActions.openSpawnFile(spawnPath);
      await spawnActions.exportSpawnFile(outputPath);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to unpack file:", error);
    } finally {
      await spawnActions.closeSpawnFile();
    }
  }, [spawnPath, outputPath]);

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
        <Typography>Select *.spawn file to unpack</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} container item>
          <OutlinedInput
            ref={inputRef}
            size={"small"}
            disabled={isSelecting || spawnFile.isLoading}
            value={spawnPath ?? ""}
            placeholder={"Source"}
            readOnly={true}
            error={Boolean(spawnFile.error)}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectSpawnFile}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            sx={{ mb: 1 }}
            onClick={onSelectSpawnFileClicked}
          />

          <OutlinedInput
            ref={inputRef}
            size={"small"}
            disabled={isSelecting || spawnFile.isLoading}
            value={outputPath ?? ""}
            placeholder={"Destination"}
            readOnly={true}
            error={Boolean(spawnFile.error)}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectOutput}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectOutputClicked}
          />
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button
            disabled={!spawnPath || !outputPath || isSelecting || spawnFile.isLoading}
            variant={"contained"}
            onClick={onUnpackClicked}
          >
            Unpack
          </Button>
        </Grid>
      </Grid>

      {spawnFile.error ? (
        <Grid>
          <FormHelperText error>{String(spawnFile.error)}</FormHelperText>
        </Grid>
      ) : null}

      {isFinishedSuccessfully ? (
        <Grid p={"0 8px"}>
          <Alert severity={"success"} variant={"outlined"}>
            Successfully unpacked spawn to {outputPath}
          </Alert>
        </Grid>
      ) : null}

      {spawnFile.isLoading ? <CircularProgress size={24} /> : null}

      <SpawnBackButton disabled={spawnFile.isLoading || isSelecting} />
    </Grid>
  );
}
