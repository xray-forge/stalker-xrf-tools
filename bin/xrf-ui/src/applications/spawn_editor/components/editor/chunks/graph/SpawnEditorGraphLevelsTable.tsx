import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { AnyObject } from "@/core/types/general";
import { IGraphLevel } from "@/lib/spawn_file";

interface ISpawnEditorGraphLevelsTableProps {
  levels: Array<IGraphLevel>;
}

export function SpawnEditorGraphLevelsTable({ levels }: ISpawnEditorGraphLevelsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "name", headerName: "name", width: 160 },
      { field: "section", headerName: "section" },
      { field: "guid", headerName: "guid", width: 240 },
      {
        field: "offset",
        headerName: "offset",
        valueGetter: (it: AnyObject) => (it.value ? JSON.stringify(it.value) : null),
      },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => levels, [levels]);

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
