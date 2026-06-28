import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as FactCheckIcon } from "@mui/icons-material/FactCheck";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as FormatAlignLeftIcon } from "@mui/icons-material/FormatAlignLeft";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ConfigsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Explorer", icon: <FolderOpenIcon />, to: "/configs_editor/explorer" },
      { label: "Verifier", icon: <FactCheckIcon />, to: "/configs_editor/verifier" },
      { label: "Formatter", icon: <FormatAlignLeftIcon />, to: "/configs_editor/formatter" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF configs editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/config_editor.html"}
      items={items}
    />
  );
}
