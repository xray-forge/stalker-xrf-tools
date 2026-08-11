import { default as FactCheckIcon } from "@mui/icons-material/FactCheck";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as FormatAlignLeftIcon } from "@mui/icons-material/FormatAlignLeft";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ConfigsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Explorer",
        description: "Browse LTX configuration files",
        icon: <FolderOpenIcon />,
        to: "/configs-editor/explorer",
      },
      {
        label: "Verifier",
        description: "Validate LTX configuration files",
        icon: <FactCheckIcon />,
        to: "/configs-editor/verifier",
      },
      {
        label: "Formatter",
        description: "Check or format LTX configuration files",
        icon: <FormatAlignLeftIcon />,
        to: "/configs-editor/formatter",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
