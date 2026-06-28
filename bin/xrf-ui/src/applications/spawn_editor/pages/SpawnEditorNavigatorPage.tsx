import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function SpawnEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/spawn_editor/editor" },
      { label: "Unpack", icon: <UnarchiveIcon />, to: "/spawn_editor/unpack" },
      { label: "Pack", icon: <Inventory2Icon />, to: "/spawn_editor/pack" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF spawn editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/spawn_editor.html"}
      items={items}
    />
  );
}
