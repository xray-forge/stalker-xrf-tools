import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Box, Button, Grid, IconButton, InputAdornment, OutlinedInput, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { useCallback, useState } from "react";

import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";

export function ConfigsEditorExplorerPage() {
  const { xrfConfigsPath } = useInjection(ProjectService);

  const [configsPath] = useState<Optional<string>>(xrfConfigsPath);

  const onSelectTargetDirectory = useCallback(() => {}, []);

  const onSelectTargetDirectoryClicked = useCallback(() => {}, []);

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column",
        width: "100%",
        height: "100%",
      }}
    >
      <Grid container sx={{ justifyContent: "center", marginBottom: 2 }}>
        <Typography>Provide LTX files directory to open</Typography>
      </Grid>

      <Grid container sx={{ justifyContent: "center", width: "auto", marginBottom: 2 }}>
        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto", marginRight: 1 }}>
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
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button variant={"contained"}>Open</Button>
        </Box>
      </Grid>

      <ApplicationBackButton path={"/configs_editor"} />
    </Box>
  );
}
