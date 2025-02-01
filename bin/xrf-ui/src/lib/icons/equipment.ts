export interface IEquipmentSectionDescriptor {
  section: string;
  w: number;
  h: number;
  x: number;
  y: number;
}

export interface IEquipmentResponse {
  systemLtxPath: string;
  path: string;
  name: string;
  equipmentDescriptors: Array<IEquipmentSectionDescriptor>;
}

export type TEquipmentCell = [number, number];

export interface IPackEquipmentResult {
  duration: number;
  savedAt: string;
  savedWidth: number;
  savedHeight: number;
  packedCount: number;
  skippedCount: number;
}
