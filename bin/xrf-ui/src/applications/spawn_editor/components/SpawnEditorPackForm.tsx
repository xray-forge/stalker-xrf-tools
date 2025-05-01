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
import { open, save } from "@tauri-apps/plugin-dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectUnpackedAllSpawnPath, getProjectAllSpawnRepackPath } from "@/lib/xrf_path";

export function SpawnEditorPackForm({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("spawn-pack");

  const [isSelecting, setIsSelecting] = useState(false);
  const [isFinishedSuccessfully, setIsFinishedSuccessfully] = useState(false);
  const [spawnPath, setSpawnPath] = useState<Optional<string>>(null);
  const [inputPath, setInputPath] = useState<Optional<string>>(null);

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

        const selected: Optional<string> = (await save({
          title: "Select spawn file output",
          filters: [{ name: "spawn", extensions: ["spawn"] }],
        })) as Optional<string>;

        setSpawnPath(selected);

        log.info("Selected file output:", selected);
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

  const onSelectInput = useCallback(
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
          title: "Select unpacked spawn folder",
          directory: true,
        })) as Optional<string>;

        setInputPath(selected);

        log.info("Selected output:", selected);
      } finally {
        setIsSelecting(false);
      }
    },
    [spawnFile]
  );

  const onSelectInputClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectInput(event),
    [onSelectSpawnFile]
  );

  const onPackClicked = useCallback(async () => {
    log.info("Packing path:", inputPath, spawnPath);

    setIsFinishedSuccessfully(false);

    if (!spawnPath || !inputPath) {
      return log.error("Cannot pack file, expected correct paths:", spawnPath, inputPath);
    }

    try {
      await spawnActions.importSpawnFile(inputPath);
      await spawnActions.saveSpawnFile(spawnPath);

      setIsFinishedSuccessfully(true);
    } catch (error) {
      log.error("Failed to pack file:", error);
    } finally {
      await spawnActions.closeSpawnFile();
    }
  }, [spawnPath, inputPath]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectUnpackedAllSpawnPath(xrfProjectPath).then((inputPath) => {
        setInputPath(inputPath);
      });

      getProjectAllSpawnRepackPath(xrfProjectPath).then((outputPath) => setSpawnPath(outputPath));
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
        <Typography>Select *.spawn file to unpack</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} gap={1} container>
          <OutlinedInput
            size={"small"}
            disabled={isSelecting || spawnFile.isLoading}
            value={inputPath ?? ""}
            placeholder={"Source"}
            readOnly={true}
            error={Boolean(spawnFile.error)}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectInputClicked}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            onClick={onSelectInput}
          />

          <OutlinedInput
            size={"small"}
            disabled={isSelecting || spawnFile.isLoading}
            value={spawnPath ?? ""}
            placeholder={"Output spawn"}
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
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container>
          <Button
            disabled={!spawnPath || !inputPath || isSelecting || spawnFile.isLoading}
            variant={"contained"}
            onClick={onPackClicked}
          >
            Pack
          </Button>
        </Grid>
      </Grid>

      {spawnFile.error ? (
        <Grid>
          <FormHelperText error>{String(spawnFile.error)}</FormHelperText>
        </Grid>
      ) : null}

      {isFinishedSuccessfully ? (
        <Grid p={"0 8px"} maxWidth={500}>
          <Alert severity={"success"} variant={"outlined"}>
            Successfully packed spawn to {spawnPath}
          </Alert>
        </Grid>
      ) : null}

      {spawnFile.isLoading ? <CircularProgress size={24} /> : null}

      <ApplicationBackButton disabled={spawnFile.isLoading || isSelecting} path={"/spawn_editor"} />
    </Grid>
  );
}
