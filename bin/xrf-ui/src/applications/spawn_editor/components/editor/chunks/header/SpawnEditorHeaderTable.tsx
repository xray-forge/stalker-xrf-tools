import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { ISpawnFileHeaderChunk } from "@/lib/spawn_file";

interface ISpawnEditorHeaderTableProps {
  header: ISpawnFileHeaderChunk;
}

export function SpawnEditorHeaderTable({ header }: ISpawnEditorHeaderTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "version", headerName: "version" },
      { field: "objects_count", headerName: "objects_count" },
      { field: "level_count", headerName: "level_count" },
      { field: "guid", headerName: "guid" },
      { field: "graph_guid", headerName: "graph_guid" },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => [{ ...header, id: "main" }], []);

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
