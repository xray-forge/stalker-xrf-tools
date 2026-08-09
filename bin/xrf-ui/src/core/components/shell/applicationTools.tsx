import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as MapIcon } from "@mui/icons-material/Map";
import { default as SettingsApplicationsIcon } from "@mui/icons-material/SettingsApplications";
import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { ReactElement } from "react";

export interface IApplicationTool {
  label: string;
  description: string;
  icon: ReactElement;
  path: string;
}

/**
 * The tool roster, shared by the rail and the welcome page so they can never disagree.
 */
export const APPLICATION_TOOLS: Array<IApplicationTool> = [
  {
    label: "Archives",
    description: "Browse and unpack game archives",
    icon: <ArchiveIcon />,
    path: "/archives_editor",
  },
  { label: "Dialogs", description: "Edit NPC dialog graphs", icon: <ForumIcon />, path: "/dialog_editor" },
  {
    label: "Configs",
    description: "Explore, verify and format LTX",
    icon: <SettingsApplicationsIcon />,
    path: "/configs_editor",
  },
  { label: "Exports", description: "Inspect script exports", icon: <SwapHorizIcon />, path: "/exports_editor" },
  { label: "Icons", description: "Edit equipment and icon sprites", icon: <ImageIcon />, path: "/icons_editor" },
  { label: "Spawns", description: "Inspect spawn files", icon: <MapIcon />, path: "/spawn_editor" },
  {
    label: "Translations",
    description: "Manage localization tables",
    icon: <TranslateIcon />,
    path: "/translations_editor",
  },
  { label: "Visuals", description: "Preview models and animations", icon: <ViewInArIcon />, path: "/visuals_editor" },
];
