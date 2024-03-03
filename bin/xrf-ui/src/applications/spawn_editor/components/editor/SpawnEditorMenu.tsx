import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as CottageIcon } from "@mui/icons-material/Cottage";
import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as MemoryIcon } from "@mui/icons-material/Memory";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { dialog } from "@tauri-apps/api";
import { useManager } from "dreamstate";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, redirect, useNavigate } from "react-router-dom";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { Optional } from "@/core/types/general";

export function SpawnEditorMenu({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onSaveClicked = useCallback(async () => {
    const path: Optional<string> = await dialog.save({
      title: "Save spawn file",
      filters: [{ name: "spawn", extensions: ["spawn"] }],
    });

    if (path) {
      await spawnActions.saveSpawnFile(path);
    }
  }, [spawnActions, redirect]);

  const onExportClicked = useCallback(async () => {
    const path: Optional<string> = (await dialog.open({
      title: "Export spawn file",
      directory: true,
    })) as Optional<string>;

    if (path) {
      await spawnActions.exportSpawnFile(path);
    }
  }, [spawnActions, redirect]);

  const onCloseClicked = useCallback(() => {
    navigate("general", { replace: true });

    return spawnActions.closeSpawnFile();
  }, [spawnActions, redirect]);

  return (
    <Drawer variant={"permanent"} open={true} sx={{ height: "100%" }} PaperProps={{ sx: { position: "relative" } }}>
      <List>
        <ListItem disablePadding>
          <ListItemButton onClick={() => navigate("general", { replace: true })}>
            <ListItemIcon>
              <CottageIcon />
            </ListItemIcon>
            <ListItemText primary={"General"} />
          </ListItemButton>
        </ListItem>
      </List>

      <Divider />

      <List>
        {["Header", "Alife", "Artefacts", "Patrols", "Graph"].map((text) => (
          <ListItem key={text} disablePadding>
            <ListItemButton onClick={() => navigate(text.toLowerCase(), { replace: true })}>
              <ListItemIcon>
                <MemoryIcon />
              </ListItemIcon>
              <ListItemText primary={text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <Divider />

      <List>
        <ListItem disablePadding>
          <ListItemButton disabled={spawnFile.isLoading} onClick={onSaveClicked}>
            <ListItemIcon>
              <SaveIcon />
            </ListItemIcon>
            <ListItemText primary={"Save"} />
          </ListItemButton>
        </ListItem>

        <ListItem disablePadding>
          <ListItemButton disabled={spawnFile.isLoading} onClick={onExportClicked}>
            <ListItemIcon>
              <ImportExportIcon />
            </ListItemIcon>
            <ListItemText primary={"Export"} />
          </ListItemButton>
        </ListItem>

        <ListItem disablePadding>
          <ListItemButton disabled={spawnFile.isLoading} onClick={onCloseClicked}>
            <ListItemIcon>
              <CloseIcon />
            </ListItemIcon>
            <ListItemText primary={"Close"} />
          </ListItemButton>
        </ListItem>
      </List>
    </Drawer>
  );
}
