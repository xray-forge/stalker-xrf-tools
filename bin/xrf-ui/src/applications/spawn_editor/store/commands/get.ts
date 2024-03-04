import { invoke } from "@tauri-apps/api/tauri";

import { Optional } from "@/core/types/general";
import { ECommand } from "@/lib/ipc";
import { ISpawnFileHeaderChunk } from "@/lib/spawn_file";

export async function getSpawnFileHeader(): Promise<Optional<ISpawnFileHeaderChunk>> {
  return await invoke(ECommand.EXPORT_SPAWN_FILE);
}
