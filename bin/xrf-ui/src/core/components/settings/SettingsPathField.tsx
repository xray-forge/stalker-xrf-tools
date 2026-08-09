import { default as ClearIcon } from "@mui/icons-material/Clear";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, IconButton, TextField, Tooltip, Typography } from "@mui/material";
import { ReactElement } from "react";

import { Optional } from "@/core/types/general";

export interface ISettingsPathFieldProps {
  label: string;
  description: string;
  value: Optional<string>;
  onSelect: () => void;
  onClear: () => void;
}

/**
 * One directory setting: what it is, what it is set to, and the two things you can do to it.
 *
 * The value is rendered monospaced because these are always filesystem paths and are compared by eye.
 */
export function SettingsPathField({
  label,
  description,
  value,
  onSelect,
  onClear,
}: ISettingsPathFieldProps): ReactElement {
  return (
    <Box>
      <Typography variant={"subtitle2"}>{label}</Typography>

      <Typography variant={"caption"} sx={{ display: "block", color: "text.secondary", marginBottom: 1 }}>
        {description}
      </Typography>

      <TextField
        fullWidth
        size={"small"}
        placeholder={"Not selected"}
        value={value ?? ""}
        sx={{ "& .MuiInputBase-input": { fontFamily: "'Cascadia Mono', 'Consolas', monospace", fontSize: "0.75rem" } }}
        slotProps={{
          input: {
            readOnly: true,
            sx: { cursor: "pointer" },
            endAdornment: (
              <Box sx={{ display: "flex", flexShrink: 0 }}>
                {value ? (
                  <Tooltip title={"Clear"}>
                    <IconButton
                      onClick={(event) => {
                        event.stopPropagation();
                        onClear();
                      }}
                    >
                      <ClearIcon fontSize={"small"} />
                    </IconButton>
                  </Tooltip>
                ) : null}

                <Tooltip title={"Choose directory"}>
                  <IconButton onClick={onSelect}>
                    <FolderOpenIcon fontSize={"small"} />
                  </IconButton>
                </Tooltip>
              </Box>
            ),
          },
        }}
        onClick={onSelect}
      />
    </Box>
  );
}
