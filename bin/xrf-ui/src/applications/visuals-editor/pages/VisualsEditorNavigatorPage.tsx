import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function VisualsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Visual preview",
        description: "Preview a game visual in 3D",
        icon: <ViewInArIcon />,
        to: "/visuals-editor/visual-preview",
      },
      {
        label: "Project visuals",
        description: "Browse visuals referenced by a project",
        icon: <AccountTreeIcon />,
        to: "/visuals-editor/visual-project",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
