import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { identifierColumn, textColumn } from "@/core/components/table";
import { Patrol } from "@/lib/bindings/xray-db";

interface IPatrolRow {
  name: string;
  pointsCount: number;
  linksCount: number;
}

interface ISpawnEditorPatrolsTableProps {
  patrols: Array<Patrol>;
}

export function SpawnEditorPatrolsTable({ patrols }: ISpawnEditorPatrolsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      identifierColumn("name", "Patrol", 320),
      textColumn("pointsCount", "Points", 110),
      textColumn("linksCount", "Links", 110),
    ],
    []
  );

  const rows: Array<IPatrolRow> = useMemo(
    () =>
      patrols.map((it: Patrol) => ({
        linksCount: it.links.length,
        name: it.name,
        pointsCount: it.points.length,
      })),
    [patrols]
  );

  return (
    <SpawnTable<IPatrolRow>
      columns={columns}
      rows={rows}
      countNoun={"patrol"}
      emptyLabel={"This file defines no patrols."}
      source={"Patrol"}
      getRowId={(row: IPatrolRow): GridRowId => row.name}
      getSearchText={(row: IPatrolRow): string => row.name}
    />
  );
}
