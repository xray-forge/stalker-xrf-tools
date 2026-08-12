import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, IconButton, InputAdornment, TextField, Tooltip, Typography } from "@mui/material";
import { ChangeEvent, KeyboardEvent, ReactElement } from "react";

import { BaseComponentProps } from "@/lib/dom/element-types";

interface IArchivesMenuHeaderProps extends BaseComponentProps {
  fileCount: number;
  query: string;
  onClear: () => void;
  /** Lets the search field drive the result list without losing focus. */
  onKeyDown?: (event: KeyboardEvent<HTMLElement>) => void;
  onQueryChange: (query: string) => void;
}

export function ArchivesMenuHeader({
  fileCount,
  query,
  onClear,
  onKeyDown,
  onQueryChange,
}: IArchivesMenuHeaderProps): ReactElement {
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
        <Typography variant={"subtitle2"}>Files</Typography>
        <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
          {fileCount}
        </Typography>
      </Box>

      <TextField
        value={query}
        placeholder={"Filter files"}
        slotProps={{
          htmlInput: {
            "aria-label": "Filter archive files",
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
                  <IconButton aria-label={"Clear file filter"} edge={"end"} onClick={onClear}>
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
