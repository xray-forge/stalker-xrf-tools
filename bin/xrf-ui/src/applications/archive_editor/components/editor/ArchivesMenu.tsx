import { default as CloseIcon } from "@mui/icons-material/Close";
import { Divider, Drawer, Grid, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { RichTreeView, TreeViewBaseItem } from "@mui/x-tree-view";
import { useManager } from "dreamstate";
import { ReactElement, SyntheticEvent, useCallback, useMemo } from "react";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { Optional } from "@/core/types/general";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu({
  archivesContext: { project: { value: project, isLoading }, archiveActions, fileActions } = useManager(
    ArchivesManager
  ),
}): ReactElement {
  const items: Array<TreeViewBaseItem> = useMemo(
    () => parseTree(Object.values(project?.files ?? {}), "\\"),
    [project?.files]
  );

  const onSelectListItem = useCallback(
    (_: SyntheticEvent, file: Optional<string>) => {
      if (file) {
        // trim '~/' root
        return fileActions.open(file.slice(2));
      }
    },
    [fileActions]
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
        <RichTreeView items={items} onSelectedItemsChange={onSelectListItem} />
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
