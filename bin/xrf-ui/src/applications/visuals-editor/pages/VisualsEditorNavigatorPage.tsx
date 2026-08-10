import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function VisualsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Visual preview", icon: <ViewInArIcon />, to: "/visuals_editor/visual_preview" },
      { label: "Project visuals", icon: <AccountTreeIcon />, to: "/visuals_editor/visual_project" },
    ],
    []
  );

  return (
    <ToolNavigator
      items={items}
    />
  );
}
