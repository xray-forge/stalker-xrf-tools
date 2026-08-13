import { invoke, isTauri } from "@tauri-apps/api/core";

import { Logger } from "@/lib/logging";

/**
 * Tell the backend to drop whatever the editor had open, on the way out.
 */
export function releaseEditorProject(command: string): void {
  if (!isTauri()) {
    return;
  }

  invoke(command).catch((error: unknown) => {
    Logger.error("Failed to release editor project on deactivation:", command, error);
  });
}
