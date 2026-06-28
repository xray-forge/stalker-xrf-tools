import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as InventoryIcon } from "@mui/icons-material/Inventory2";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function IconsEditorNavigatorPage(): ReactElement {
  const items = useMemo(
    () => [
      { label: "Equipment editor", icon: <ImageIcon />, to: "/icons_editor/icons_equipment" },
      { label: "Equipment pack", icon: <InventoryIcon />, to: "/icons_editor/icons_equipment_pack" },
      { label: "Equipment unpack", icon: <InventoryIcon />, to: "/icons_editor/icons_equipment_unpack" },
      { label: "Description editor", icon: <DescriptionIcon />, to: "/icons_editor/icons_description" },
      { label: "Description pack", icon: <DescriptionIcon />, to: "/icons_editor/icons_description_pack" },
      { label: "Description unpack", icon: <DescriptionIcon />, to: "/icons_editor/icons_description_unpack" },
      { label: "Back", icon: <ArrowBackIcon />, to: "/", isSecondary: true },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF icons editor"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/icon_editor.html"}
      items={items}
    />
  );
}
