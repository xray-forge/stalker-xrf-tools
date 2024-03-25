import { default as CloseIcon } from "@mui/icons-material/Close";
import { Divider, Drawer, Grid, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { RichTreeView, TreeViewBaseItem } from "@mui/x-tree-view";
import { useManager } from "dreamstate";
import { ReactElement, useMemo } from "react";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu({
  archivesContext: { project: { value: project, isLoading }, archiveActions } = useManager(ArchivesManager),
}): ReactElement {
  const items: Array<TreeViewBaseItem> = useMemo(
    () => parseTree(Object.values(project?.files ?? {}), "\\"),
    [project?.files]
  );

  return (
    <Drawer
      variant={"permanent"}
      open={true}
      sx={{ height: "100%", width: 320 }}
      PaperProps={{ sx: { position: "relative" } }}
    >
      <List disablePadding>
        <ListItem disablePadding>
          <ListItemButton>
            <ListItemText primary={"Files"} />
          </ListItemButton>
        </ListItem>
      </List>

      <Grid padding={1} flexGrow={1} overflow={"auto"}>
        <RichTreeView items={items} />
      </Grid>

      <Divider />

      <List disablePadding>
        <ListItem disablePadding>
          <ListItemButton disabled={isLoading} onClick={archiveActions.close}>
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
