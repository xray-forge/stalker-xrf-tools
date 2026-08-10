import { Box, TextField, Typography } from "@mui/material";
import { DataGrid, GridColDef } from "@mui/x-data-grid";
import { ReactElement, useMemo, useState } from "react";

const GRID_HEIGHT: number = 420;

export interface ICommandResultFindingsProps<T> {
  rows: Array<T>;
  columns: Array<GridColDef>;
  getRowId: (row: T) => string | number;
  /** Everything about a row that a search should match, flattened into one string. */
  getSearchText: (row: T) => string;
  emptyLabel: string;
  searchPlaceholder?: string;
}

/**
 * The findings a command produced, as a sortable and filterable table.
 *
 * A grid rather than a hand rolled list because a real gamedata verify emits thousands of rows: the
 * previous list rendered every one of them into a 300px box with no virtualisation. `@mui/x-data-grid`
 * is already a dependency, already used by the spawn editor, and already themed to compact density.
 *
 * Filtering is implemented here rather than through the grid's own toolbar, whose API has moved
 * between versions. One text field over `getSearchText` is both stable and enough.
 */
export function CommandResultFindings<T>({
  rows,
  columns,
  getRowId,
  getSearchText,
  emptyLabel,
  searchPlaceholder = "Filter findings",
}: ICommandResultFindingsProps<T>): ReactElement {
  const [search, setSearch] = useState<string>("");

  const filtered: Array<T> = useMemo(() => {
    const query: string = search.trim().toLowerCase();

    return query ? rows.filter((row) => getSearchText(row).toLowerCase().includes(query)) : rows;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rows, search]);

  if (!rows.length) {
    return (
      <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
        {emptyLabel}
      </Typography>
    );
  }

  return (
    <Box sx={{ display: "flex", flexDirection: "column", gap: 1, width: "100%" }}>
      <TextField
        size={"small"}
        placeholder={searchPlaceholder}
        value={search}
        sx={{ maxWidth: 320 }}
        onChange={(event) => setSearch(event.target.value)}
      />

      <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
        {filtered.length === rows.length
          ? `${rows.length} finding(s)`
          : `${filtered.length} of ${rows.length} finding(s)`}
      </Typography>

      <Box sx={{ height: GRID_HEIGHT, width: "100%" }}>
        <DataGrid rows={filtered} columns={columns} getRowId={getRowId} disableColumnMenu />
      </Box>
    </Box>
  );
}
