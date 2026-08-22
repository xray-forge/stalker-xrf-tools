import { default as AddIcon } from "@mui/icons-material/Add";
import { default as DeleteIcon } from "@mui/icons-material/Delete";
import { Box, Button, FormControlLabel, IconButton, Stack, Switch, TextField, Typography } from "@mui/material";
import { ChangeEvent, ReactElement } from "react";

import { withDirectoryAt, withoutAt } from "@/applications/archives-packer/lib/pack-config";
import { ArchivePackDirectory } from "@/core/bindings/types/xrf-pack";

interface IPackerDirectoryListProps {
  directories: Array<ArchivePackDirectory>;
  isDisabled?: boolean;
  addLabel: string;
  emptyLabel: string;
  /** What the per-row switch means, which differs between including and excluding. */
  recursiveLabel: string;
  onChange: (directories: Array<ArchivePackDirectory>) => void;
}

/**
 * Editable list of directory rules, each a path relative to the packed root plus its recursive flag.
 */
export function PackerDirectoryList({
  directories,
  isDisabled,
  addLabel,
  emptyLabel,
  recursiveLabel,
  onChange,
}: IPackerDirectoryListProps): ReactElement {
  return (
    <Stack spacing={1}>
      {directories.length ? (
        directories.map((directory, index) => (
          <Stack key={index} direction={"row"} spacing={1} sx={{ alignItems: "center" }}>
            <TextField
              size={"small"}
              fullWidth
              disabled={isDisabled}
              value={directory.path}
              // An empty path is the packed root itself rather than a missing value.
              placeholder={"root of the source directory"}
              slotProps={{ htmlInput: { "aria-label": `Directory ${index + 1}` } }}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                onChange(withDirectoryAt(directories, index, { path: event.target.value }))
              }
            />

            <FormControlLabel
              sx={{ flexShrink: 0, mr: 0 }}
              control={
                <Switch
                  size={"small"}
                  disabled={isDisabled}
                  checked={directory.isRecursive}
                  slotProps={{ input: { "aria-label": `${recursiveLabel} for ${directory.path || "the root"}` } }}
                  onChange={(event: ChangeEvent<HTMLInputElement>) =>
                    onChange(withDirectoryAt(directories, index, { isRecursive: event.target.checked }))
                  }
                />
              }
              label={<Typography variant={"body2"}>{recursiveLabel}</Typography>}
            />

            <IconButton
              aria-label={`Remove ${directory.path || "the root"}`}
              size={"small"}
              disabled={isDisabled}
              onClick={() => onChange(withoutAt(directories, index))}
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
        <Button
          size={"small"}
          disabled={isDisabled}
          startIcon={<AddIcon />}
          onClick={() => onChange([...directories, { path: "", isRecursive: true }])}
        >
          {addLabel}
        </Button>
      </Box>
    </Stack>
  );
}
