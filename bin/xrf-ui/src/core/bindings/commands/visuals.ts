// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import { SelectedVisualDescription, SubmeshTexture, VisualSource } from "@/core/bindings/types/xrf-app";
import { VisualDescription } from "@/core/bindings/types/xrf-visual";

/** Commands */
export const visualsCommands = {
  /** Drop the selected visual and its packed geometry. */
  closeModel: () => __TAURI_INVOKE<null>("plugin:visuals|close_model"),
  /**
   * What the viewer had selected, or null when nothing is open.
   *
   * This is the rehydration probe: a reloaded frontend asks what is selected and then asks for that
   * source's geometry, so the selection survives a reload without the frontend storing anything.
   */
  getModel: () =>
    __TAURI_INVOKE<{
      source: VisualSource;
      description: VisualDescription;
      textures: Array<SubmeshTexture>;
    } | null>("plugin:visuals|get_model"),
  /**
   * Select a visual and return what it contains.
   *
   * Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather
   * than repeating it. The bytes are not returned: a typed command cannot carry them, which is why they
   * are read separately.
   */
  openModel: (source: VisualSource, fallbackRoot: string | null) =>
    __TAURI_INVOKE<SelectedVisualDescription>("plugin:visuals|open_model", { source, fallbackRoot }),
};
