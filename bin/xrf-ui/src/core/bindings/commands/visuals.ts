// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

import { AssetWorldSpec, SelectedVisualDescription, VisualSource } from "@/core/bindings/types/xrf-app";
import { VisualDependencies, VisualDescription } from "@/core/bindings/types/xrf-visual";

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
      dependencies: VisualDependencies;
    } | null>("plugin:visuals|get_model"),
  /**
   * Select a visual and return what it contains, with every reference it declares resolved.
   *
   * Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather than repeating
   * it. The bytes are not returned: a typed command cannot carry them, which is why they are read separately.
   *
   * Resolution happens once, for the whole dependency set, in this one call. That is what keeps a model with forty
   * textures from costing forty round trips, and it is why the outcomes travel with the description rather than being
   * asked for afterwards.
   */
  openModel: (source: VisualSource, world: AssetWorldSpec) =>
    __TAURI_INVOKE<SelectedVisualDescription>("plugin:visuals|open_model", { source, world }),
};
