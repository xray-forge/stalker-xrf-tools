import { default as FolderIcon } from "@mui/icons-material/Folder";
import { FormControl, IconButton, InputAdornment, InputLabel, OutlinedInput } from "@mui/material";
import { MouseEvent, ReactElement } from "react";

import { Optional } from "@/core/types/general";

export interface IFilePickerInputProps {
  value?: Optional<string>;
  label: string;
  size?: "small";
  disabled?: boolean;
  onClick?: (event: MouseEvent<HTMLElement>) => void;
}

export function FilePickerInput({
  label,
  size = "small",
  value,
  disabled,
  onClick,
}: IFilePickerInputProps): ReactElement {
  return (
    <FormControl size={size} variant={"outlined"} disabled={disabled}>
      <InputLabel size={size}>{label}</InputLabel>
      <OutlinedInput
        readOnly
        size={size}
        type={"text"}
        endAdornment={
          <InputAdornment position={"end"} onClick={onClick}>
            <IconButton edge={"end"} disabled={disabled}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        label={label}
        value={value || ""}
        onClick={onClick}
      />
    </FormControl>
  );
}
