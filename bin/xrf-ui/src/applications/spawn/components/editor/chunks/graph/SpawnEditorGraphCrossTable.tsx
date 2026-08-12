import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { identifierColumn, textColumn } from "@/core/components/table";
import { GraphCrossTable } from "@/lib/xrf/bindings/xray-db";

interface ISpawnEditorGraphCrossTableProps {
  crossTables: Array<GraphCrossTable>;
}

export function SpawnEditorGraphCrossTable({ crossTables }: ISpawnEditorGraphCrossTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      identifierColumn("levelGuid", "Level guid", 260),
      identifierColumn("gameGuid", "Game guid", 260),
      textColumn("version", "Version", 100),
      textColumn("nodesCount", "Nodes", 110),
      textColumn("verticesCount", "Vertices", 110),
    ],
    []
  );

  return (
    <SpawnTable<GraphCrossTable>
      columns={columns}
      countNoun={"cross table"}
      emptyLabel={"This graph has no cross tables."}
      rows={crossTables}
      source={"Graph cross table"}
      getRowId={(row: GraphCrossTable): GridRowId => row.levelGuid}
      getSearchText={(row: GraphCrossTable): string => row.levelGuid}
    />
  );
}
