import { IEquipmentSectionDescriptor } from "@/lib/icons/equipment";

export class GridMapper {
  public rows: number;
  public columns: number;
  public grid: Array<Array<Array<IEquipmentSectionDescriptor>>>;
  public gridSize: number;

  public constructor(width: number, height: number, size: number, descriptors: Array<IEquipmentSectionDescriptor>) {
    const rows: number = Math.floor(height / size);
    const columns: number = Math.floor(width / size);
    const grid: Array<Array<Array<IEquipmentSectionDescriptor>>> = new Array(rows);

    for (let it = 0; it < rows; it++) {
      grid[it] = new Array(columns).fill(null);
    }

    descriptors.forEach((it) => {
      if (it.x + it.w <= columns && it.y + it.h <= rows) {
        for (let i = it.x; i < it.x + it.w; i++) {
          for (let j = it.y; j < it.y + it.h; j++) {
            if (grid[j][i]) {
              grid[j][i].push(it);
            } else {
              grid[j][i] = [it];
            }
          }
        }
      }
    });

    this.grid = grid;
    this.rows = rows;
    this.columns = columns;
    this.gridSize = size;
  }
}
