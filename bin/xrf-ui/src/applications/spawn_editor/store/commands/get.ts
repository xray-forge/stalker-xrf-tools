import { invoke } from "@tauri-apps/api/tauri";

import { ISpawnFileHeader } from "@/applications/spawn_editor/types";
import { Optional } from "@/core/types/general";
import { ECommand } from "@/lib/ipc";

export async function getSpawnFileHeader(): Promise<Optional<ISpawnFileHeader>> {
  return await invoke(ECommand.EXPORT_SPAWN_FILE);
}
