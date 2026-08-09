import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ExportsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/exports_editor/exports" },
    ],
    []
  );

  return (
    <ToolNavigator
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/exports_editor.html"}
      items={items}
    />
  );
}
