import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as SearchIcon } from "@mui/icons-material/Search";
import { Box, Chip, IconButton, InputAdornment, TextField, Tooltip } from "@mui/material";
import { ChangeEvent, KeyboardEvent, ReactElement, RefObject } from "react";

import { EApplicationGroupId, IApplicationGroup } from "@/core/routing/application";
import { Nullable } from "@/lib/types/general";

export interface IApplicationLauncherGroupFilter {
  group: IApplicationGroup;
  count: number;
}

export interface IApplicationLauncherFiltersProps {
  filters: ReadonlyArray<IApplicationLauncherGroupFilter>;
  /** Lets the launcher's keyboard shortcut reach the field it does not own. */
  inputRef: RefObject<Nullable<HTMLInputElement>>;
  query: string;
  /** `null` is every group rather than none. */
  selectedGroupId: Nullable<EApplicationGroupId>;
  totalCount: number;
  onClear: () => void;
  onKeyDown: (event: KeyboardEvent<HTMLElement>) => void;
  onQueryChange: (query: string) => void;
  onSelectGroup: (groupId: Nullable<EApplicationGroupId>) => void;
}

/**
 * Narrowing controls for the catalog: one text query and one group at a time.
 */
export function ApplicationLauncherFilters({
  filters,
  inputRef,
  query,
  selectedGroupId,
  totalCount,
  onClear,
  onKeyDown,
  onQueryChange,
  onSelectGroup,
}: IApplicationLauncherFiltersProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", gap: 1.25 }}>
      <TextField
        value={query}
        placeholder={"Search tools"}
        inputRef={inputRef}
        sx={{ width: "100%", maxWidth: 320 }}
        slotProps={{
          htmlInput: {
            "aria-label": "Search tools",
          },
          input: {
            startAdornment: (
              <InputAdornment position={"start"}>
                <SearchIcon fontSize={"small"} />
              </InputAdornment>
            ),
            endAdornment: query ? (
              <InputAdornment position={"end"}>
                <Tooltip title={"Clear search"}>
                  <IconButton aria-label={"Clear tool search"} edge={"end"} onClick={onClear}>
                    <ClearIcon fontSize={"small"} />
                  </IconButton>
                </Tooltip>
              </InputAdornment>
            ) : (
              <InputAdornment position={"end"}>
                <Box
                  aria-hidden={true}
                  sx={{
                    paddingX: 0.5,
                    color: "text.secondary",
                    border: "1px solid",
                    borderColor: "divider",
                    borderRadius: 1,
                    fontSize: "0.625rem",
                    lineHeight: "16px",
                    whiteSpace: "nowrap",
                  }}
                >
                  Ctrl K
                </Box>
              </InputAdornment>
            ),
          },
        }}
        onKeyDown={onKeyDown}
        onChange={(event: ChangeEvent<HTMLInputElement>) => onQueryChange(event.target.value)}
      />

      <Box sx={{ display: "flex", flexWrap: "wrap", gap: 0.75 }}>
        <Chip
          size={"small"}
          label={`All ${totalCount}`}
          aria-pressed={selectedGroupId === null}
          color={selectedGroupId === null ? "primary" : "default"}
          variant={selectedGroupId === null ? "filled" : "outlined"}
          onClick={() => onSelectGroup(null)}
        />

        {filters.map(({ group, count }: IApplicationLauncherGroupFilter) => {
          const isSelected: boolean = selectedGroupId === group.id;

          return (
            <Chip
              key={group.id}
              size={"small"}
              label={`${group.label} ${count}`}
              aria-pressed={isSelected}
              color={isSelected ? "primary" : "default"}
              variant={isSelected ? "filled" : "outlined"}
              // Clicking the active chip is how someone gets back to everything without aiming at `All`.
              onClick={() => onSelectGroup(isSelected ? null : group.id)}
            />
          );
        })}
      </Box>
    </Box>
  );
}
