// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  closeEquipmentSprite: () => __TAURI_INVOKE<null>("plugin:icons-editor|close_equipment_sprite"),
  getEquipmentSprite: () =>
    __TAURI_INVOKE<{
      path: string;
      name: string;
      systemLtxPath: string;
      equipmentDescriptors: Array<InventorySpriteDescriptor>;
    } | null>("plugin:icons-editor|get_equipment_sprite"),
  openEquipmentSprite: (equipmentDdsPath: string, systemLtxPath: string) =>
    __TAURI_INVOKE<IconsEditorEquipmentResponse>("plugin:icons-editor|open_equipment_sprite", {
      equipmentDdsPath,
      systemLtxPath,
    }),
  reopenEquipmentSprite: () =>
    __TAURI_INVOKE<IconsEditorEquipmentResponse>("plugin:icons-editor|reopen_equipment_sprite"),
  packEquipment: (sourcePath: string, outputPath: string, systemLtxPath: string) =>
    __TAURI_INVOKE<PackEquipmentResult>("plugin:icons-editor|pack_equipment", {
      sourcePath,
      outputPath,
      systemLtxPath,
    }),
};

/* Types */
export type IconsEditorEquipmentResponse = {
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
