import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { RichTreeView, TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";
import { ReactElement, useMemo } from "react";

/**
 * Left side panel listing visuals available in a gamedata or resources tree.
 *
 * The tree below is a hardcoded sample of the real `meshes` layout. Nothing is wired: picking a project
 * directory and walking it belongs to the rust side, which will supply the same item shape the archive
 * editor already uses for its file tree.
 */
export function VisualProjectTree(): ReactElement {
  const items: Array<TreeViewDefaultItemModelProperties> = useMemo(
    () => [
      {
        id: "meshes",
        label: "meshes",
        children: [
          {
            id: "meshes\\dynamics",
            label: "dynamics",
            children: [
              {
                id: "meshes\\dynamics\\weapons",
                label: "weapons",
                children: [
                  { id: "meshes\\dynamics\\weapons\\wpn_ak74.ogf", label: "wpn_ak74.ogf" },
                  { id: "meshes\\dynamics\\weapons\\wpn_ak74_hud.ogf", label: "wpn_ak74_hud.ogf" },
                ],
              },
            ],
          },
          {
            id: "meshes\\actors",
            label: "actors",
            children: [{ id: "meshes\\actors\\stalker.ogf", label: "stalker.ogf" }],
          },
        ],
      },
    ],
    []
  );

  return (
    <Drawer
      variant={"permanent"}
      open={true}
      sx={{ height: "100%", width: 280, flexShrink: 0 }}
      slotProps={{ paper: { sx: { position: "relative", width: 280 } } }}
    >
      <List disablePadding>
        <ListItem disablePadding>
          <ListItemButton disabled>
            <ListItemIcon>
              <FolderOpenIcon />
            </ListItemIcon>
            <ListItemText primary={"Open project"} secondary={"Needs backend"} />
          </ListItemButton>
        </ListItem>
      </List>

      <Divider />

      <Box sx={{ padding: 1, flexGrow: 1, minHeight: 0, overflow: "auto" }}>
        <RichTreeView items={items} />
      </Box>
    </Drawer>
  );
}
