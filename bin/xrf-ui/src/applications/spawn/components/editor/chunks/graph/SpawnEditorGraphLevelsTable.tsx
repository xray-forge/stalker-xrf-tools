import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { identifierColumn, textColumn, vectorColumn } from "@/core/components/table";
import { GraphLevel } from "@/lib/rust-sdk/xray-db";

interface ISpawnEditorGraphLevelsTableProps {
  levels: Array<GraphLevel>;
}

export function SpawnEditorGraphLevelsTable({ levels }: ISpawnEditorGraphLevelsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      textColumn("id", "Id", 80),
      identifierColumn("name", "Name", 200),
      identifierColumn("section", "Section", 180),
      vectorColumn("offset", "Offset"),
      identifierColumn("guid", "Guid", 260),
    ],
    []
  );

  return (
    <SpawnTable<GraphLevel>
      columns={columns}
      rows={levels}
      countNoun={"level"}
      emptyLabel={"This graph has no levels."}
      source={"Graph level"}
      getRowId={(row: GraphLevel): GridRowId => row.id}
      getSearchText={(row: GraphLevel): string => `${row.name} ${row.section}`}
    />
  );
}
