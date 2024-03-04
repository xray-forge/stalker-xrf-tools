import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IGraphCrossTable } from "@/lib/spawn_file";

interface ISpawnEditorGraphCrossTableProps {
  crossTables: Array<IGraphCrossTable>;
}

export function SpawnEditorGraphCrossTable({ crossTables }: ISpawnEditorGraphCrossTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "gameGuid", headerName: "gam guid", width: 240 },
      { field: "levelGuid", headerName: "level guid", width: 240 },
      { field: "version", headerName: "version" },
      { field: "nodesCount", headerName: "nodes count" },
      { field: "verticesCount", headerName: "vertices count", width: 120 },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => crossTables.map((it) => ({ ...it, id: it.levelGuid })), [crossTables]);

  return (
    <DataGrid
      rowHeight={24}
      rows={rows}
      columns={columns}
      sx={{
        maxWidth: "100%",
      }}
      slots={{ toolbar: TableToolbar }}
      slotProps={{
        toolbar: {
          showQuickFilter: true,
        },
      }}
      initialState={{
        pagination: { paginationModel: { pageSize: 50 } },
      }}
    />
  );
}
