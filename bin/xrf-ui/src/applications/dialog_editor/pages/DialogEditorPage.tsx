import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function DialogEditorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Open", icon: <FolderOpenIcon />, to: "/dialog_editor/todo" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF dialog editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/dialog_editor.html"}
      items={items}
    />
  );
}
