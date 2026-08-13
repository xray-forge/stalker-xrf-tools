import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn/components/editor/table/SpawnTable";
import { decimalColumn, textColumn, vectorColumn } from "@/core/components/table";
import { ArtefactSpawnPoint } from "@/lib/xrf/bindings/xrf-db";

interface IArtefactNodeRow extends ArtefactSpawnPoint {
  index: number;
}

interface ISpawnEditorArtefactsNodesTableProps {
  nodes: Array<ArtefactSpawnPoint>;
}

export function SpawnEditorArtefactsNodesTable({ nodes }: ISpawnEditorArtefactsNodesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      textColumn("index", "#", 70),
      textColumn("levelVertexId", "Level vertex", 130),
      decimalColumn("distance", "Distance"),
      vectorColumn("position", "Position"),
    ],
    []
  );

  const rows: Array<IArtefactNodeRow> = useMemo(
    () => nodes.map((it: ArtefactSpawnPoint, index: number) => ({ ...it, index })),
    [nodes]
  );

  return (
    <SpawnTable<IArtefactNodeRow>
      columns={columns}
      countNoun={"node"}
      emptyLabel={"This file spawns no artefacts."}
      rows={rows}
      source={"Artefact spawn node"}
      getRowId={(row: IArtefactNodeRow): GridRowId => row.index}
    />
  );
}
