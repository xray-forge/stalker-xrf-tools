import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function DialogEditorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Open",
        description: "Browse and edit game dialogs",
        icon: <FolderOpenIcon />,
        to: "/dialog-editor/todo",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
