// Auto-generated rust bindings. Do not edit it manually.

import { VisualSource } from "@/core/bindings/xrf-app-visuals";
import { invokeRaw } from "@/core/ipc/raw";

/** Commands answering with raw bytes, which Specta cannot type. */
export const commands = {
  readGeometry: (source: VisualSource): Promise<ArrayBuffer> => invokeRaw("plugin:visuals|read_geometry", { source }),
  readTexture: (source: VisualSource, reference: string, fallbackRoot: string | null): Promise<ArrayBuffer> =>
    invokeRaw("plugin:visuals|read_texture", { source, reference, fallbackRoot }),
};
