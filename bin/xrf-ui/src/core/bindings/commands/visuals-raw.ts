// Auto-generated rust bindings. Do not edit it manually.

import { VisualSource } from "@/core/bindings/types/xrf-app";
import { invokeRaw } from "@/core/ipc/raw";

/** Commands answering with raw bytes, which Specta cannot type. */
export const visualsRawCommands = {
  readGeometry: (source: VisualSource): Promise<ArrayBuffer> => invokeRaw("plugin:visuals|read_geometry", { source }),
};
