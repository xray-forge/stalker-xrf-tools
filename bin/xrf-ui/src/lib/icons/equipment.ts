export interface IEquipmentSectionDescriptor {
  name: string;
  w: number;
  h: number;
  x: number;
  y: number;
}

export interface IEquipmentResponse {
  path: string;
  name: string;
  equipmentDescriptors: Array<IEquipmentSectionDescriptor>;
}
