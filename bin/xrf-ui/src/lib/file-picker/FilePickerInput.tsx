import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, IconButton, TextField, Tooltip } from "@mui/material";
import { ReactElement } from "react";

import { Optional } from "@/core/types/general";

export interface IFilePickerInputProps {
  label?: string;
  placeholder?: string;
  description?: string;
  value?: Optional<string>;
  isDisabled?: boolean;
  isInvalid?: boolean;
  onSelect: () => void;
  onClear?: () => void;
}

/**
 * One path to pick, in the compact form every open and unpack screen uses.
 */
export function FilePickerInput({
  label,
  placeholder = "Not selected",
  description,
  value,
  isDisabled,
  isInvalid,
  onSelect,
  onClear,
}: IFilePickerInputProps): ReactElement {
  return (
    <TextField
      fullWidth
      size={"small"}
      label={label}
      placeholder={placeholder}
      helperText={description}
      disabled={isDisabled}
      error={isInvalid}
      value={value ?? ""}
      sx={{ "& .MuiInputBase-input": { fontFamily: "'Cascadia Mono', 'Consolas', monospace", fontSize: "0.75rem" } }}
      slotProps={{
        input: {
          readOnly: true,
          sx: { cursor: isDisabled ? "default" : "pointer" },
          endAdornment: (
            <Box sx={{ display: "flex", flexShrink: 0 }}>
              {value && onClear ? (
                <Tooltip title={"Clear"}>
                  <IconButton
                    disabled={isDisabled}
                    onClick={(event) => {
                      event.stopPropagation();
                      onClear();
                    }}
                  >
                    <ClearIcon fontSize={"small"} />
                  </IconButton>
                </Tooltip>
              ) : null}

              <Tooltip title={"Choose"}>
                {/* Stops the click reaching the field's own handler, which would open a second dialog. */}
                <IconButton
                  disabled={isDisabled}
                  onClick={(event) => {
                    event.stopPropagation();
                    onSelect();
                  }}
                >
                  <FolderOpenIcon fontSize={"small"} />
                </IconButton>
              </Tooltip>
            </Box>
          ),
        },
      }}
      onClick={isDisabled ? undefined : onSelect}
    />
  );
}
