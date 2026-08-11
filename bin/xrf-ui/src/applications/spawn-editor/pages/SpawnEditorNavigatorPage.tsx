import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function SpawnEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Open",
        description: "Browse and edit a packed spawn file",
        icon: <FolderOpenIcon />,
        to: "/spawn-editor/editor",
      },
      {
        label: "Unpack",
        description: "Extract a spawn file into editable chunks",
        icon: <UnarchiveIcon />,
        to: "/spawn-editor/unpack",
      },
      {
        label: "Pack",
        description: "Build a spawn file from unpacked chunks",
        icon: <Inventory2Icon />,
        to: "/spawn-editor/pack",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
