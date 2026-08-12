import { GridColDef, GridRowId } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { SpawnTable } from "@/applications/spawn-editor/components/editor/table/SpawnTable";
import { decimalColumn, textColumn, vectorColumn } from "@/core/components/table";
import { IArtefactSpawnNode } from "@/lib/spawn-file";

interface IArtefactNodeRow extends IArtefactSpawnNode {
  index: number;
}

interface ISpawnEditorArtefactsNodesTableProps {
  nodes: Array<IArtefactSpawnNode>;
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
    () => nodes.map((it: IArtefactSpawnNode, index: number) => ({ ...it, index })),
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
