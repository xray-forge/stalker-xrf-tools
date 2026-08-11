import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, IconButton, InputAdornment, TextField, Tooltip, Typography } from "@mui/material";
import { ChangeEvent, KeyboardEvent, ReactElement } from "react";

import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IExportsMenuHeaderProps extends BaseComponentProps {
  exportCount: number;
  query: string;
  onClear: () => void;
  /** Lets the search field drive the result list without losing focus. */
  onKeyDown?: (event: KeyboardEvent<HTMLElement>) => void;
  onQueryChange: (query: string) => void;
}

export function ExportsMenuHeader({
  exportCount,
  query,
  onClear,
  onKeyDown,
  onQueryChange,
}: IExportsMenuHeaderProps): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        gap: 1,
        padding: 1,
        borderBottom: 1,
        borderColor: "divider",
      }}
    >
      <Box sx={{ display: "flex", alignItems: "baseline", justifyContent: "space-between", paddingX: 0.5 }}>
        <Typography variant={"subtitle2"}>Exports</Typography>
        <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
          {exportCount}
        </Typography>
      </Box>

      <TextField
        value={query}
        placeholder={"Filter exports"}
        slotProps={{
          htmlInput: {
            "aria-label": "Filter exports",
          },
          input: {
            startAdornment: (
              <InputAdornment position={"start"}>
                <SearchIcon fontSize={"small"} />
              </InputAdornment>
            ),
            endAdornment: query ? (
              <InputAdornment position={"end"}>
                <Tooltip title={"Clear filter"}>
                  <IconButton aria-label={"Clear export filter"} edge={"end"} onClick={onClear}>
                    <ClearIcon fontSize={"small"} />
                  </IconButton>
                </Tooltip>
              </InputAdornment>
            ) : null,
          },
        }}
        onKeyDown={onKeyDown}
        onChange={(event: ChangeEvent<HTMLInputElement>) => onQueryChange(event.target.value)}
      />
    </Box>
  );
}
