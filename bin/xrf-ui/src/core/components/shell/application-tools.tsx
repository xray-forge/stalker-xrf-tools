import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as MapIcon } from "@mui/icons-material/Map";
import { default as SettingsApplicationsIcon } from "@mui/icons-material/SettingsApplications";
import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { ReactElement } from "react";

import { Nullable } from "@/core/types/general";

/**
 * Stable identity of a tool, independent of its route and its display name.
 *
 * Notification records carry one of these rather than a label, so a renamed tool renames its own
 * history instead of leaving two spellings of it in the log.
 */
export enum EApplicationToolId {
  ARCHIVES = "archives",
  CONFIGS = "configs",
  DIALOGS = "dialogs",
  EXPORTS = "exports",
  ICONS = "icons",
  SPAWNS = "spawns",
  TRANSLATIONS = "translations",
  VISUALS = "visuals",
}

/** Sources that raise notifications without owning a tool of their own. */
export const APPLICATION_SOURCE: string = "application";

export interface IApplicationTool {
  id: EApplicationToolId;
  label: string;
  title: string;
  description: string;
  icon: ReactElement;
  path: string;
}

/**
 * Resolve the tool owning a route.
 *
 * Every surface that needs to name the current tool goes through here, so the rail, the toolbar and
 * the navigator cannot drift into three different names for one thing.
 */
export function findApplicationTool(pathname: string): Nullable<IApplicationTool> {
  return APPLICATION_TOOLS.find((tool) => pathname.startsWith(tool.path)) ?? null;
}

/** Resolve a tool from what a notification recorded as its source. */
export function findApplicationToolById(id: string): Nullable<IApplicationTool> {
  return APPLICATION_TOOLS.find((tool) => tool.id === id) ?? null;
}

/**
 * The tool roster: the single source for how a tool is named anywhere in the application.
 *
 * `label` is the short form the rail and status bar use, `title` the long form for headers. The `XRF`
 * prefix is deliberately absent - it is already the window title, and repeating it inside the app was
 * noise.
 */
export const APPLICATION_TOOLS: Array<IApplicationTool> = [
  {
    id: EApplicationToolId.ARCHIVES,
    label: "Archives",
    title: "Archives editor",
    description: "Browse and unpack game archives",
    icon: <ArchiveIcon />,
    path: "/archives-editor",
  },
  {
    id: EApplicationToolId.DIALOGS,
    label: "Dialogs",
    title: "Dialog editor",
    description: "Edit NPC dialog graphs",
    icon: <ForumIcon />,
    path: "/dialog-editor",
  },
  {
    id: EApplicationToolId.CONFIGS,
    label: "Configs",
    title: "Configs editor",
    description: "Explore, verify and format LTX",
    icon: <SettingsApplicationsIcon />,
    path: "/configs-editor",
  },
  {
    id: EApplicationToolId.EXPORTS,
    label: "Exports",
    title: "Exports editor",
    description: "Inspect script exports",
    icon: <SwapHorizIcon />,
    path: "/exports-editor",
  },
  {
    id: EApplicationToolId.ICONS,
    label: "Icons",
    title: "Icons editor",
    description: "Edit equipment and icon sprites",
    icon: <ImageIcon />,
    path: "/icons-editor",
  },
  {
    id: EApplicationToolId.SPAWNS,
    label: "Spawns",
    title: "Spawn editor",
    description: "Inspect spawn files",
    icon: <MapIcon />,
    path: "/spawn-editor",
  },
  {
    id: EApplicationToolId.TRANSLATIONS,
    label: "Translations",
    title: "Translations editor",
    description: "Manage localization tables",
    icon: <TranslateIcon />,
    path: "/translations-editor",
  },
  {
    id: EApplicationToolId.VISUALS,
    label: "Visuals",
    title: "Visuals editor",
    description: "Preview models and animations",
    icon: <ViewInArIcon />,
    path: "/visuals-editor",
  },
];
