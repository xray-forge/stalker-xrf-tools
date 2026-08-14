import { invoke, isTauri } from "@tauri-apps/api/core";

import { Logger } from "@/lib/logging";

/**
 * Tell the backend to drop whatever the editor had open, on the way out.
 *
 * @param release - Tauri command name or generated release command to invoke.
 */
export function releaseEditorProject(release: string | (() => Promise<unknown>)): void {
  if (!isTauri()) {
    return;
  }

  const request: Promise<unknown> = typeof release === "string" ? invoke(release) : release();

  request.catch((error: unknown) => {
    Logger.error("Failed to release editor project on deactivation:", release, error);
  });
}
