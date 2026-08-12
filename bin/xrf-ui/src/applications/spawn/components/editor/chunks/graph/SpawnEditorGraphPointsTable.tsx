import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { decimalColumn, textColumn } from "@/core/components/table";
import { IGraphPoint } from "@/lib/spawn-file";

interface IGraphPointRow extends IGraphPoint {
  index: number;
}

interface ISpawnEditorGraphPointsTableProps {
  points: Array<IGraphPoint>;
}

export function SpawnEditorGraphPointsTable({ points }: ISpawnEditorGraphPointsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      textColumn("index", "#", 90),
      textColumn("levelVertexId", "Level vertex", 140),
      decimalColumn("distance", "Distance", 130),
    ],
    []
  );

  const rows: Array<IGraphPointRow> = useMemo(
    () => points.map((it: IGraphPoint, index: number) => ({ ...it, index })),
    [points]
  );

  return (
    <SpawnTable<IGraphPointRow>
      columns={columns}
      rows={rows}
      countNoun={"point"}
      emptyLabel={"This graph has no points."}
      source={"Graph point"}
      getRowId={(row: IGraphPointRow): GridRowId => row.index}
    />
  );
}
