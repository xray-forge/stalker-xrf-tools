import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, CircularProgress, Grid, IconButton, InputAdornment, OutlinedInput, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { useManager } from "dreamstate";
import { useCallback, useState } from "react";

import { ConfigsBackButton } from "@/applications/configs_editor/components/ConfigsBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { ECommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";

export function ConfigsEditorFormatterPage({ projectContext: { xrfConfigsPath } = useManager(ProjectManager) }) {
  const log: Logger = useLogger("configs-formatter");

  const [isLoading, setIsLoading] = useState(false);
  const [configsPath] = useState<Optional<string>>(xrfConfigsPath);

  const onSelectTargetDirectory = useCallback(() => {}, []);

  const onSelectTargetDirectoryClicked = useCallback(() => {}, []);

  const onFormatPathClicked = useCallback(async () => {
    try {
      setIsLoading(true);

      log.info("Formatting:", configsPath);

      const result = await invoke(ECommand.FORMAT_CONFIGS_PATH, { path: configsPath });

      log.info("Formatted:", configsPath, result);
    } catch (error) {
      log.error("Format error:", error);
    } finally {
      setIsLoading(false);
    }
  }, [configsPath, log]);

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
        <Typography>Provide LTX files directory to format</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} container item>
          <OutlinedInput
            size={"small"}
            disabled={isLoading}
            value={configsPath || ""}
            placeholder={"Configs directory"}
            readOnly={true}
            endAdornment={
              <InputAdornment position={"end"} onClick={onSelectTargetDirectory}>
                <IconButton edge={"end"}>
                  <FolderIcon />
                </IconButton>
              </InputAdornment>
            }
            sx={{ mb: 1 }}
            onClick={onSelectTargetDirectoryClicked}
          />
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button variant={"contained"} disabled={isLoading} onClick={onFormatPathClicked}>
            Format
          </Button>
        </Grid>
      </Grid>

      {isLoading ? <CircularProgress size={24} /> : null}

      <ConfigsBackButton disabled={isLoading} />
    </Grid>
  );
}
