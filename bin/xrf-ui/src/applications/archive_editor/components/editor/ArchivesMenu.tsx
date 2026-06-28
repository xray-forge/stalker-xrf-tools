import { default as CloseIcon } from "@mui/icons-material/Close";
import { Box, Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { RichTreeView, TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";
import { useInjection } from "@wirestate/react";
import { ReactElement, SyntheticEvent, useCallback, useMemo } from "react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { Optional } from "@/core/types/general";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const items: Array<TreeViewDefaultItemModelProperties> = useMemo(
    () => parseTree(Object.values(archivesService.project.value?.files ?? {}), "\\"),
    [archivesService.project.value?.files]
  );

  const onSelectListItem = useCallback(
    (_: Optional<SyntheticEvent>, file: Optional<string>) => {
      if (file) {
        // trim '~/' root
        return archivesService.openArchiveFile(file.slice(2));
      }
    },
    [archivesService]
  );

  return (
    <Drawer
      variant={"permanent"}
      open={true}
      sx={{ height: "100%", width: 320 }}
      slotProps={{ paper: { sx: { position: "relative" } } }}
    >
      <List disablePadding>
        <ListItem disablePadding>
          <ListItemButton>
            <ListItemText primary={"Files"} />
          </ListItemButton>
        </ListItem>
      </List>

      <Box sx={{ padding: 1, flexGrow: 1, overflow: "auto" }}>
        <RichTreeView items={items} onSelectedItemsChange={onSelectListItem} />
      </Box>

      <Divider />

      <List disablePadding>
        <ListItem disablePadding>
          <ListItemButton disabled={archivesService.project.isLoading} onClick={archivesService.closeArchivesProject}>
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
