import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function TranslationsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/translations_editor/project" },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF translations editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/translations_editor.html"}
      items={items}
    />
  );
}
