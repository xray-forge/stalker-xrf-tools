// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeSprite: () => __TAURI_INVOKE<null>("plugin:equipment-icons|close_sprite"),
  getSprite: () =>
    __TAURI_INVOKE<{
      path: string;
      name: string;
      systemLtxPath: string;
      equipmentDescriptors: Array<InventorySpriteDescriptor>;
    } | null>("plugin:equipment-icons|get_sprite"),
  openSprite: (equipmentDdsPath: string, systemLtxPath: string) =>
    __TAURI_INVOKE<EquipmentSpriteMetadata>("plugin:equipment-icons|open_sprite", { equipmentDdsPath, systemLtxPath }),
  reopenSprite: () => __TAURI_INVOKE<EquipmentSpriteMetadata>("plugin:equipment-icons|reopen_sprite"),
  packSprite: (sourcePath: string, outputPath: string, systemLtxPath: string) =>
    __TAURI_INVOKE<PackEquipmentResult>("plugin:equipment-icons|pack_sprite", {
      sourcePath,
      outputPath,
      systemLtxPath,
    }),
};

/* Types */
export type EquipmentSpriteMetadata = {
  path: string;
  name: string;
  systemLtxPath: string;
  equipmentDescriptors: Array<InventorySpriteDescriptor>;
};

export type InventorySpriteDescriptor = {
  section: string;
  customIcon: string | null;
  x: number;
  y: number;
  w: number;
  h: number;
};

export type PackEquipmentResult = {
  duration: number;
  savedAt: string;
  savedWidth: number;
  savedHeight: number;
  packedCount: number;
  skippedCount: number;
};
