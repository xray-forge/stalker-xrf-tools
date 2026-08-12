import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { textColumn, tupleColumn, vectorColumn } from "@/core/components/table";
import { IGraphVertex } from "@/lib/spawn-file";

/** Offsets locate a vertex inside the file rather than in the world; available, off by default. */
const HIDDEN_COLUMNS: Array<string> = ["edgesOffset", "levelPointOffset"];

interface IGraphVertexRow extends IGraphVertex {
  index: number;
}

interface ISpawnEditorGraphVerticesTableProps {
  vertices: Array<IGraphVertex>;
}

export function SpawnEditorGraphVerticesTable({ vertices }: ISpawnEditorGraphVerticesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      textColumn("index", "#", 90),
      textColumn("levelId", "Level", 90),
      textColumn("levelVertexId", "Level vertex", 130),
      vectorColumn("gamePoint", "Game point"),
      vectorColumn("levelPoint", "Level point"),
      textColumn("edgesCount", "Edges", 100),
      // Was `levelPointsCount`, which the vertex does not carry - the column read empty in every file.
      textColumn("levelPointCount", "Points", 100),
      tupleColumn("vertexType", "Vertex type"),
      textColumn("edgesOffset", "Edges offset", 130),
      textColumn("levelPointOffset", "Points offset", 130),
    ],
    []
  );

  const rows: Array<IGraphVertexRow> = useMemo(
    () => vertices.map((it: IGraphVertex, index: number) => ({ ...it, index })),
    [vertices]
  );

  return (
    <SpawnTable<IGraphVertexRow>
      columns={columns}
      rows={rows}
      countNoun={"vertex"}
      emptyLabel={"This graph has no vertices."}
      hiddenColumns={HIDDEN_COLUMNS}
      source={"Graph vertex"}
      getRowId={(row: IGraphVertexRow): GridRowId => row.index}
    />
  );
}
