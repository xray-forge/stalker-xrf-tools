import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function VisualsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Visual preview", icon: <ViewInArIcon />, to: "/visuals_editor/visual_preview" },
      { label: "Project visuals", icon: <AccountTreeIcon />, to: "/visuals_editor/visual_project" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF visuals editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/app.html"}
      items={items}
    />
  );
}
