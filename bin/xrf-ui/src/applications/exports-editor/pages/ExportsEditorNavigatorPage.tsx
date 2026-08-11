import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function ExportsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Open",
        description: "Browse typescript extern declarations from an XRF project",
        icon: <FolderOpenIcon />,
        to: "/exports-editor/exports",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
