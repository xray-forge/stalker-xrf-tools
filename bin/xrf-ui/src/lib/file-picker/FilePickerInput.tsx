import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, IconButton, TextField, Tooltip } from "@mui/material";
import { ReactElement } from "react";

import { Optional } from "@/core/types/general";
import { FormRow } from "@/lib/form/FormRow";

export interface IFilePickerInputProps {
  /** When given, the control labels itself by composing a `FormRow`. */
  label?: string;
  description?: string;
  isRequired?: boolean;
  error?: Optional<string>;
  placeholder?: string;
  value?: Optional<string>;
  isDisabled?: boolean;
  isInvalid?: boolean;
  onSelect: () => void;
  onClear?: () => void;
}

/**
 * The control half of a path row. Labelling and description belong to the surrounding `FormRow`.
 * The value is monospaced because these are filesystem paths, compared by eye.
 */
export function FilePickerInput({
  label,
  description,
  isRequired,
  error,
  placeholder = "Not selected",
  value,
  isDisabled,
  isInvalid,
  onSelect,
  onClear,
}: IFilePickerInputProps): ReactElement {
  const control: ReactElement = (
    <TextField
      fullWidth
      size={"small"}
      placeholder={placeholder}
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

  return label ? (
    <FormRow label={label} description={description} isRequired={isRequired} error={error}>
      {control}
    </FormRow>
  ) : (
    control
  );
}
