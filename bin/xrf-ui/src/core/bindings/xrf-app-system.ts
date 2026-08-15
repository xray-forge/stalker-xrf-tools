// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  /**
   * Show a path in the desktop's own file manager.
   *
   * This exists instead of the shell plugin's `open` because that command validates what it is handed
   * against a regex which only matches `http`, `mailto` and `tel`, so a filesystem path is always
   * rejected. Widening that scope would allow opening any file with its default handler, executables
   * included, while this only ever hands a path to the file manager.
   */
  revealPath: (path: string) => __TAURI_INVOKE<null>("plugin:system|reveal_path", { path }),
};
