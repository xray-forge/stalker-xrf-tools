import { ContainerConfig } from "@wirestate/core";
import { ComponentType, ReactElement } from "react";

/**
 * Stable identity of an application, independent of its route and its display name.
 */
export enum EApplicationId {
  ARCHIVES = "archives",
  ARCHIVES_UNPACK = "archives-unpack",
  CONFIGS_EXPLORER = "configs-explorer",
  CONFIGS_FORMAT = "configs-format",
  CONFIGS_VERIFY = "configs-verify",
  DESCRIPTION_ICONS = "description-icons",
  DESCRIPTION_ICONS_PACK = "description-icons-pack",
  DESCRIPTION_ICONS_UNPACK = "description-icons-unpack",
  DIALOGS = "dialogs",
  EQUIPMENT_ICONS = "equipment-icons",
  EQUIPMENT_ICONS_PACK = "equipment-icons-pack",
  EQUIPMENT_ICONS_UNPACK = "equipment-icons-unpack",
  EXPORTS = "exports",
  PROJECT_VISUALS = "project-visuals",
  SPAWN = "spawn",
  SPAWN_PACK = "spawn-pack",
  SPAWN_UNPACK = "spawn-unpack",
  TRANSLATIONS = "translations",
  VISUAL_PREVIEW = "visual-preview",
}

/**
 * The family an application belongs to.
 */
export enum EApplicationGroupId {
  ARCHIVES = "archives",
  CONFIGS = "configs",
  DIALOGS = "dialogs",
  EXPORTS = "exports",
  ICONS = "icons",
  SPAWNS = "spawns",
  TRANSLATIONS = "translations",
  VISUALS = "visuals",
}

/**
 * Whether an application does anything yet.
 *
 * `PLANNED` surfaces exist as signposts on the home page: the roster is the roadmap, so an unbuilt
 * screen is visible but inert rather than silently missing. Developer mode opens them anyway.
 */
export enum EApplicationStatus {
  PLANNED = "planned",
  READY = "ready",
}

export interface IApplicationDescriptor {
  id: EApplicationId;
  group: EApplicationGroupId;
  /** The one name this application answers to, everywhere. */
  label: string;
  description: string;
  icon: ReactElement;
  path: string;
  status: EApplicationStatus;
  /** The container this application's services live in. Omit it to run in the root one. */
  container?: Omit<ContainerConfig, "parent">;
  Component: ComponentType;
  /** Pulls this application's chunk in before it is navigated to. */
  preload?: () => Promise<unknown>;
}

export interface IApplicationGroup {
  id: EApplicationGroupId;
  label: string;
  icon: ReactElement;
}

/** Sources that raise notifications without owning an application of their own. */
export const APPLICATION_SOURCE: string = "application";
