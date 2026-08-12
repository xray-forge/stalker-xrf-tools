import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { flagsColumn, identifierColumn, textColumn, vectorColumn } from "@/core/components/table";
import { Patrol, PatrolPoint } from "@/lib/bindings/xray-db";

interface IPatrolPointRow extends PatrolPoint {
  id: string;
  patrol: string;
}

interface ISpawnEditorPatrolPointsTableProps {
  patrols: Array<Patrol>;
}

export function SpawnEditorPatrolPointsTable({ patrols }: ISpawnEditorPatrolPointsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      identifierColumn("patrol", "Patrol", 300),
      identifierColumn("name", "Point", 180),
      flagsColumn("flags", "Flags"),
      textColumn("levelVertexId", "Level vertex", 130),
      textColumn("gameVertexId", "Game vertex", 130),
      vectorColumn("position", "Position"),
    ],
    []
  );

  const rows: Array<IPatrolPointRow> = useMemo(
    () =>
      patrols.flatMap((patrol: Patrol) =>
        patrol.points.map((point: PatrolPoint) => ({
          ...point,
          id: `${patrol.name}/${point.name}`,
          patrol: patrol.name,
        }))
      ),
    [patrols]
  );

  return (
    <SpawnTable<IPatrolPointRow>
      columns={columns}
      rows={rows}
      countNoun={"point"}
      emptyLabel={"These patrols have no points."}
      source={"Patrol point"}
      getRowId={(row: IPatrolPointRow): GridRowId => row.id}
      getSearchText={(row: IPatrolPointRow): string => `${row.patrol} ${row.name}`}
    />
  );
}
