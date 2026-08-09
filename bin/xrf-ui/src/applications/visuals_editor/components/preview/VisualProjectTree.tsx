import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box } from "@mui/material";
import { RichTreeView, TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";
import { ReactElement, useMemo } from "react";

import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";

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

  const actions: Array<IEditorSideMenuItem> = useMemo(
    () => [{ label: "Open project", description: "Needs backend", icon: <FolderOpenIcon />, isDisabled: true }],
    []
  );

  return (
    <EditorSideMenu actions={actions}>
      <Box sx={{ padding: 1 }}>
        <RichTreeView items={items} />
      </Box>
    </EditorSideMenu>
  );
}
