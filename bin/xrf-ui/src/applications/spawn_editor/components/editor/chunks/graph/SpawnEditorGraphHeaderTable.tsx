import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IGraphHeader } from "@/lib/spawn_file";

interface ISpawnEditorGraphHeaderTableProps {
  header: IGraphHeader;
}

export function SpawnEditorGraphHeaderTable({ header }: ISpawnEditorGraphHeaderTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "guid", headerName: "guid", width: 240 },
      { field: "version", headerName: "version" },
      { field: "level_count", headerName: "level_count" },
      { field: "edges_count", headerName: "edges_count" },
      { field: "point_count", headerName: "point_count" },
      { field: "vertex_count", headerName: "vertex_count" },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => [{ ...header, id: header.guid }], [header]);

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
