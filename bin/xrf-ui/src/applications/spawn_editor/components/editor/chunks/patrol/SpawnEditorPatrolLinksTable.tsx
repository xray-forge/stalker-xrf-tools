import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { AnyObject } from "@/core/types/general";
import { IPatrol } from "@/lib/spawn_file";

interface ISpawnEditorPatrolsTableProps {
  patrols: Array<IPatrol>;
}

export function SpawnEditorPatrolLinksTable({ patrols }: ISpawnEditorPatrolsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "patrol", headerName: "patrol", width: 300 },
      { field: "index", headerName: "index" },
      {
        field: "links",
        headerName: "links",
        width: 160,
        valueGetter: (it: AnyObject) => (it.row.links ? JSON.stringify(it.row.links) : null),
      },
    ],
    [patrols]
  );

  const rows: GridRowsProp = useMemo(() => {
    const links = [];

    for (const patrol of patrols) {
      for (const link of patrol.links) {
        links.push({
          ...link,
          id: patrol.name + link.index,
          patrol: patrol.name,
        });
      }
    }

    return links;
  }, [patrols]);

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
