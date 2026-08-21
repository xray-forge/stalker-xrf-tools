import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn-editor/components/editor/table/SpawnTable";
import { GraphEdge } from "@/core/bindings/types/xrf-db";
import { decimalColumn, textColumn } from "@/core/ui/table";

interface IGraphEdgeRow extends GraphEdge {
  index: number;
}

interface ISpawnEditorGraphEdgesTableProps {
  edges: Array<GraphEdge>;
}

export function SpawnEditorGraphEdgesTable({ edges }: ISpawnEditorGraphEdgesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      textColumn("index", "#", 90),
      textColumn("gameVertexId", "Game vertex", 140),
      decimalColumn("distance", "Distance", 130),
    ],
    []
  );

  const rows: Array<IGraphEdgeRow> = useMemo(
    () => edges.map((it: GraphEdge, index: number) => ({ ...it, index })),
    [edges]
  );

  return (
    <SpawnTable<IGraphEdgeRow>
      columns={columns}
      rows={rows}
      countNoun={"edge"}
      emptyLabel={"This graph has no edges."}
      source={"Graph edge"}
      getRowId={(row: IGraphEdgeRow): GridRowId => row.index}
    />
  );
}
