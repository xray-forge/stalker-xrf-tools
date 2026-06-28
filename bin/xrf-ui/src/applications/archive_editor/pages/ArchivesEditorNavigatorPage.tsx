import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ArchivesEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/archives_editor/editor" },
      { label: "Unpack", icon: <UnarchiveIcon />, to: "/archives_editor/unpacker" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF archive editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/archive_editor.html"}
      items={items}
    />
  );
}
