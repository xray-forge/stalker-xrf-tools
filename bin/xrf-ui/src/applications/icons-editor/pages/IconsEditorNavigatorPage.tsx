import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as InventoryIcon } from "@mui/icons-material/Inventory2";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function IconsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Equipment editor", icon: <ImageIcon />, to: "/icons-editor/icons-equipment" },
      { label: "Equipment pack", icon: <InventoryIcon />, to: "/icons-editor/icons-equipment-pack" },
      { label: "Equipment unpack", icon: <InventoryIcon />, to: "/icons-editor/icons-equipment-unpack" },
      { label: "Description editor", icon: <DescriptionIcon />, to: "/icons-editor/icons-description" },
      { label: "Description pack", icon: <DescriptionIcon />, to: "/icons-editor/icons-description-pack" },
      { label: "Description unpack", icon: <DescriptionIcon />, to: "/icons-editor/icons-description-unpack" },
    ],
    []
  );

  return <ToolNavigator items={items} />;
}
