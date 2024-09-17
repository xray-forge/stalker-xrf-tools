import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as SearchIcon } from "@mui/icons-material/Search";
import { debounce, FormControl, InputAdornment, TextField } from "@mui/material";
import { ChangeEvent, ReactElement, useCallback, useMemo, useState } from "react";

export interface IExportsFilterFormProps {
  debounceDelay?: number;
  onFilterValueChangeDebounced?: (value: string) => void;
  onFilterValueChange?: (value: string) => void;
}

export function ExportsFilterForm({
  debounceDelay = 250,
  onFilterValueChangeDebounced,
  onFilterValueChange,
}: IExportsFilterFormProps): ReactElement {
  const [filter, setFilter] = useState("");

  const onValueChangedDebounced = useMemo(
    () =>
      debounce(
        (value: string) => (onFilterValueChangeDebounced ? onFilterValueChangeDebounced(value) : void 0),
        debounceDelay
      ),
    [debounceDelay, onFilterValueChangeDebounced]
  );

  const onValueChanged = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    const value: string = event.target.value;

    setFilter(value);
    onValueChangedDebounced(value);

    if (onFilterValueChange) {
      onFilterValueChange(value);
    }
  }, []);

  const onClearFilter = useCallback(() => {
    setFilter("");
    onValueChangedDebounced("");

    if (onFilterValueChange) {
      onFilterValueChange("");
    }
  }, []);

  return (
    <FormControl>
      <TextField
        size={"small"}
        variant={"outlined"}
        value={filter}
        InputProps={{
          startAdornment: (
            <InputAdornment position={"start"}>
              <SearchIcon />
            </InputAdornment>
          ),
          endAdornment: (
            <InputAdornment position={"end"} onClick={onClearFilter}>
              <ClearIcon />
            </InputAdornment>
          ),
        }}
        onChange={onValueChanged}
      />
    </FormControl>
  );
}
