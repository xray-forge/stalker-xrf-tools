import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as LooksIcon3 } from "@mui/icons-material/Looks3";
import { default as LooksIcon4 } from "@mui/icons-material/Looks4";
import { default as LooksIcon5 } from "@mui/icons-material/Looks5";
import { default as LooksIcon1 } from "@mui/icons-material/LooksOne";
import { default as LooksIcon2 } from "@mui/icons-material/LooksTwo";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { dialog } from "@tauri-apps/api";
import { useManager } from "dreamstate";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, redirect, useNavigate } from "react-router-dom";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { Optional } from "@/core/types/general";

export function SpawnEditorMenu({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const sections: Array<[string, ReactElement]> = useMemo(
    () => [
      ["Header", <LooksIcon1 />],
      ["Alife", <LooksIcon2 />],
      ["Artefacts", <LooksIcon3 />],
      ["Patrols", <LooksIcon4 />],
      ["Graph", <LooksIcon5 />],
    ],
    []
  );

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
    navigate("/spawn_editor", { replace: true });

    return spawnActions.closeSpawnFile();
  }, [spawnActions, redirect]);

  return (
    <Drawer variant={"permanent"} open={true} sx={{ height: "100%" }} PaperProps={{ sx: { position: "relative" } }}>
      <List>
        {sections.map(([text, icon]) => (
          <ListItem key={text} disablePadding>
            <ListItemButton onClick={() => navigate(text.toLowerCase(), { replace: true })}>
              <ListItemIcon>{icon}</ListItemIcon>
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
