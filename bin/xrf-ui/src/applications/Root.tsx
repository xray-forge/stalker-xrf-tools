import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as MapIcon } from "@mui/icons-material/Map";
import { default as SettingsApplicationsIcon } from "@mui/icons-material/SettingsApplications";
import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { ReactElement, useMemo } from "react";

import { ToolNavigator } from "@/core/components/navigation/ToolNavigator";

export function Root(): ReactElement {
  const items = useMemo(
    () => [
      {
        label: "Archive editor",
        description: "Browse and unpack game archives",
        icon: <ArchiveIcon />,
        to: "/archives_editor",
      },
      { label: "Dialog editor", description: "Edit NPC dialog graphs", icon: <ForumIcon />, to: "/dialog_editor" },
      {
        label: "Configs editor",
        description: "Explore, verify and format LTX",
        icon: <SettingsApplicationsIcon />,
        to: "/configs_editor",
      },
      {
        label: "Exports editor",
        description: "Inspect script exports",
        icon: <SwapHorizIcon />,
        to: "/exports_editor",
      },
      {
        label: "Icon editor",
        description: "Edit equipment and icon sprites",
        icon: <ImageIcon />,
        to: "/icons_editor",
      },
      { label: "Spawn editor", description: "Inspect spawn files", icon: <MapIcon />, to: "/spawn_editor" },
      {
        label: "Translation editor",
        description: "Manage localization tables",
        icon: <TranslateIcon />,
        to: "/translations_editor",
      },
    ],
    []
  );

  return (
    <ToolNavigator
      title={"XRF development tools"}
      helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/app.html"}
      items={items}
    />
  );
}
