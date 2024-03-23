import { exists } from "@tauri-apps/api/fs";
import * as path from "@tauri-apps/api/path";

import { Optional } from "@/core/types/general";

export function getProjectConfigsPath(projectPath: string): Promise<string> {
  return path.resolve(projectPath, "src", "engine", "configs");
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

export async function getExistingProjectBuiltAllSpawnPath(projectPath: string): Promise<Optional<string>> {
  const spawnFilePath: string = await getProjectBuiltAllSpawnPath(projectPath);

  if (await exists(spawnFilePath)) {
    return spawnFilePath;
  }

  return null;
}

export async function getExistingProjectUnpackedAllSpawnPath(projectPath: string): Promise<Optional<string>> {
  const unpackedPath: string = await getProjectAllSpawnUnpackPath(projectPath);

  if (await exists(unpackedPath)) {
    return unpackedPath;
  }

  return null;
}

export async function getExistingProjectLinkedGamePath(projectPath: string): Promise<Optional<string>> {
  const unpackedPath: string = await getProjectLinkedGamePath(projectPath);

  if (await exists(unpackedPath)) {
    return unpackedPath;
  }

  return null;
}
