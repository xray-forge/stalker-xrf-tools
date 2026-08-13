import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { GraphHeader } from "@/core/bindings/xrf-db";
import { identifierColumn, textColumn } from "@/core/components/table";

interface IGraphHeaderRow extends GraphHeader {
  id: string;
}

interface ISpawnEditorGraphHeaderTableProps {
  header: GraphHeader;
}

export function SpawnEditorGraphHeaderTable({ header }: ISpawnEditorGraphHeaderTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      identifierColumn("guid", "Guid", 260),
      textColumn("version", "Version", 100),
      textColumn("levelsCount", "Levels", 110),
      textColumn("verticesCount", "Vertices", 110),
      textColumn("edgesCount", "Edges", 110),
      textColumn("pointsCount", "Points", 110),
    ],
    []
  );

  const rows: Array<IGraphHeaderRow> = useMemo(() => [{ ...header, id: header.guid }], [header]);

  return (
    <SpawnTable<IGraphHeaderRow>
      columns={columns}
      rows={rows}
      countNoun={"header"}
      emptyLabel={"This graph has no header."}
      source={"Graph header"}
      getRowId={(row: IGraphHeaderRow): GridRowId => row.id}
    />
  );
}
