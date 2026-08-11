import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as InventoryIcon } from "@mui/icons-material/Inventory2";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function IconsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Equipment editor",
        description: "Inspect and edit equipment inventory icons",
        icon: <ImageIcon />,
        to: "/icons-editor/icons-equipment",
      },
      {
        label: "Equipment pack",
        description: "Build an equipment sprite from individual icons",
        icon: <InventoryIcon />,
        to: "/icons-editor/icons-equipment-pack",
      },
      {
        label: "Equipment unpack",
        description: "Extract individual icons from an equipment sprite",
        icon: <InventoryIcon />,
        to: "/icons-editor/icons-equipment-unpack",
      },
      {
        label: "Description editor",
        description: "Inspect and edit item description icons",
        icon: <DescriptionIcon />,
        to: "/icons-editor/icons-description",
      },
      {
        label: "Description pack",
        description: "Build a description sprite from individual icons",
        icon: <DescriptionIcon />,
        to: "/icons-editor/icons-description-pack",
      },
      {
        label: "Description unpack",
        description: "Extract individual icons from a description sprite",
        icon: <DescriptionIcon />,
        to: "/icons-editor/icons-description-unpack",
      },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
