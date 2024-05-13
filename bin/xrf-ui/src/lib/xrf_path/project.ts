import { exists } from "@tauri-apps/api/fs";
import * as path from "@tauri-apps/api/path";

import { Optional } from "@/core/types/general";

export function getProjectConfigsPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "src", "engine", "configs");
}

export function getProjectExportDeclarationsPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "src", "engine", "scripts", "declarations");
}

export async function getProjectExportConditionsPath(projectPath: string): Promise<string> {
  return path.resolve(await getProjectExportDeclarationsPath(projectPath), "conditions");
}

export async function getProjectExportEffectsPath(projectPath: string): Promise<string> {
  return path.resolve(await getProjectExportDeclarationsPath(projectPath), "effects");
}

export async function getProjectExportDialogsPath(projectPath: string): Promise<string> {
  return path.resolve(await getProjectExportDeclarationsPath(projectPath), "dialogs");
}

export function getProjectBuiltAllSpawnPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "target", "gamedata", "spawns", "all.spawn");
}

export function getProjectAllSpawnUnpackPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "target", "spawns", "unpacked");
}

export function getProjectAllSpawnRepackPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "target", "spawns", "repacked", "repacked.spawn");
}

export function getProjectLinkedGamePath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "target", "game_link");
}

export function getProjectArchivesUnpackPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "target", "unpacked_archives");
}

export function getProjectEquipmentDDSPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "src", "resources", "textures", "ui", "ui_icon_equipment.dds");
}

export function getProjectEquipmentSourcePath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "src", "resources", "textures", "ui", "ui_icon_equipment");
}

export async function getProjectSystemLtxPath(projectPath: string): Promise<string> {
  return path.resolve(await getProjectConfigsPath(projectPath), "system.ltx");
}

export async function getExistingProjectBuiltAllSpawnPath(projectPath: string): Promise<Optional<string>> {
  return getPathIfExists(getProjectBuiltAllSpawnPath(projectPath));
}

export async function getExistingProjectUnpackedAllSpawnPath(projectPath: string): Promise<Optional<string>> {
  return getPathIfExists(getProjectAllSpawnUnpackPath(projectPath));
}

export async function getExistingProjectLinkedGamePath(projectPath: string): Promise<Optional<string>> {
  return getPathIfExists(getProjectLinkedGamePath(projectPath));
}

export async function getPathIfExists(path: string | Promise<string>): Promise<Optional<string>> {
  const resolved: string = await path;

  return (await exists(resolved)) ? resolved : null;
}
