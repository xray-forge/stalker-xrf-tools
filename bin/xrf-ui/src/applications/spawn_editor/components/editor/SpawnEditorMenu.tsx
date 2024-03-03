import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as CottageIcon } from "@mui/icons-material/Cottage";
import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as MemoryIcon } from "@mui/icons-material/Memory";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorMenu({
  spawnContext: { spawnActions, spawnFile } = useManager(SpawnFileManager),
}): ReactElement {
  return (
    <Drawer variant={"permanent"} open={true}>
      <List>
        <ListItem disablePadding>
          <ListItemButton>
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
            <ListItemButton>
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
          <ListItemButton disabled={spawnFile.isLoading}>
            <ListItemIcon>
              <SaveIcon />
            </ListItemIcon>
            <ListItemText primary={"Save"} />
          </ListItemButton>
        </ListItem>

        <ListItem disablePadding>
          <ListItemButton disabled={spawnFile.isLoading}>
            <ListItemIcon>
              <ImportExportIcon />
            </ListItemIcon>
            <ListItemText primary={"Export"} />
          </ListItemButton>
        </ListItem>

        <ListItem disablePadding>
          <ListItemButton disabled={spawnFile.isLoading} onClick={spawnActions.closeSpawnFile}>
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
