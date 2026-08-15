import { default as AddIcon } from "@mui/icons-material/Add";
import { default as DeleteIcon } from "@mui/icons-material/Delete";
import { Box, Button, IconButton, Stack, TextField, Typography } from "@mui/material";
import { ChangeEvent, ReactElement } from "react";

import { withoutAt, withValueAt } from "@/applications/archives-packer/lib/pack-config";

interface IPackerStringListProps {
  values: Array<string>;
  isDisabled?: boolean;
  addLabel: string;
  emptyLabel: string;
  placeholder?: string;
  onChange: (values: Array<string>) => void;
}

/**
 * Editable list of plain strings, for the sections that are just names or patterns.
 */
export function PackerStringList({
  values,
  isDisabled,
  addLabel,
  emptyLabel,
  placeholder,
  onChange,
}: IPackerStringListProps): ReactElement {
  return (
    <Stack spacing={1}>
      {values.length ? (
        values.map((value, index) => (
          <Stack key={index} direction={"row"} spacing={1} sx={{ alignItems: "center" }}>
            <TextField
              size={"small"}
              fullWidth
              disabled={isDisabled}
              value={value}
              placeholder={placeholder}
              slotProps={{ htmlInput: { "aria-label": `${addLabel} ${index + 1}` } }}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                onChange(withValueAt(values, index, event.target.value))
              }
            />

            <IconButton
              size={"small"}
              disabled={isDisabled}
              aria-label={`Remove ${value || `entry ${index + 1}`}`}
              onClick={() => onChange(withoutAt(values, index))}
            >
              <DeleteIcon fontSize={"small"} />
            </IconButton>
          </Stack>
        ))
      ) : (
        <Typography variant={"body2"} color={"text.secondary"}>
          {emptyLabel}
        </Typography>
      )}

      <Box>
        <Button size={"small"} disabled={isDisabled} startIcon={<AddIcon />} onClick={() => onChange([...values, ""])}>
          {addLabel}
        </Button>
      </Box>
    </Stack>
  );
}
