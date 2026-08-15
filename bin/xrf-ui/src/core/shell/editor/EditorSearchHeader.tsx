import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, IconButton, InputAdornment, TextField, Tooltip, Typography } from "@mui/material";
import { ChangeEvent, KeyboardEvent, ReactElement } from "react";

import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IEditorSearchHeaderProps extends BaseComponentProps {
  /** What the panel lists, as its heading. */
  title: string;
  count: number;
  query: string;
  placeholder: string;
  /** Names the field for a screen reader, which the placeholder alone does not. */
  ariaLabel: string;
  onClear: () => void;
  /** Lets the search field drive the result list without losing focus. */
  onKeyDown?: (event: KeyboardEvent<HTMLElement>) => void;
  onQueryChange: (query: string) => void;
}

/**
 * Heading and filter field for a side menu that lists more than fits on a screen.
 */
export function EditorSearchHeader({
  title,
  count,
  query,
  placeholder,
  ariaLabel,
  onClear,
  onKeyDown,
  onQueryChange,
}: IEditorSearchHeaderProps): ReactElement {
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
        <Typography variant={"subtitle2"}>{title}</Typography>
        <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
          {count}
        </Typography>
      </Box>

      <TextField
        value={query}
        placeholder={placeholder}
        slotProps={{
          htmlInput: {
            "aria-label": ariaLabel,
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
                  <IconButton aria-label={"Clear filter"} edge={"end"} onClick={onClear}>
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
