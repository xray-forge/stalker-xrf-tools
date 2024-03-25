import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, Grid, IconButton, InputAdornment, OutlinedInput, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { useCallback, useState } from "react";

import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";

export function ConfigsEditorExplorerPage({ projectContext: { xrfConfigsPath } = useManager(ProjectManager) }) {
  const [configsPath] = useState<Optional<string>>(xrfConfigsPath);

  const onSelectTargetDirectory = useCallback(() => {}, []);
  const onSelectTargetDirectoryClicked = useCallback(() => {}, []);

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
        <Typography>Provide LTX files directory to open</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} container item>
          <OutlinedInput
            size={"small"}
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
            value={configsPath || ""}
            onClick={onSelectTargetDirectoryClicked}
          />
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button variant={"contained"}>Open</Button>
        </Grid>
      </Grid>

      <ApplicationBackButton path={"/configs_editor"} />
    </Grid>
  );
}
