import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ArchivesEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/archives-editor/editor" },
      { label: "Unpack", icon: <UnarchiveIcon />, to: "/archives-editor/unpacker" },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
