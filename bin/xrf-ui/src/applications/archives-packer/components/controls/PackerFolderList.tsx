import { default as AddIcon } from "@mui/icons-material/Add";
import { default as DeleteIcon } from "@mui/icons-material/Delete";
import { Box, Button, FormControlLabel, IconButton, Stack, Switch, TextField, Typography } from "@mui/material";
import { ChangeEvent, ReactElement } from "react";

import { withFolderAt, withoutAt } from "@/applications/archives-packer/lib/pack-config";
import { ArchivePackFolder } from "@/core/bindings/types/xrf-pack";

interface IPackerFolderListProps {
  folders: Array<ArchivePackFolder>;
  isDisabled?: boolean;
  addLabel: string;
  emptyLabel: string;
  /** What the per-row switch means, which differs between including and excluding. */
  recursiveLabel: string;
  onChange: (folders: Array<ArchivePackFolder>) => void;
}

/**
 * Editable list of folder rules, each a path relative to the packed root plus its recursive flag.
 */
export function PackerFolderList({
  folders,
  isDisabled,
  addLabel,
  emptyLabel,
  recursiveLabel,
  onChange,
}: IPackerFolderListProps): ReactElement {
  return (
    <Stack spacing={1}>
      {folders.length ? (
        folders.map((folder, index) => (
          <Stack key={index} direction={"row"} spacing={1} sx={{ alignItems: "center" }}>
            <TextField
              size={"small"}
              fullWidth
              disabled={isDisabled}
              value={folder.path}
              // An empty path is the packed root itself rather than a missing value.
              placeholder={"root of the source directory"}
              slotProps={{ htmlInput: { "aria-label": `Folder ${index + 1}` } }}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                onChange(withFolderAt(folders, index, { path: event.target.value }))
              }
            />

            <FormControlLabel
              sx={{ flexShrink: 0, mr: 0 }}
              control={
                <Switch
                  size={"small"}
                  disabled={isDisabled}
                  checked={folder.isRecursive}
                  slotProps={{ input: { "aria-label": `${recursiveLabel} for ${folder.path || "the root"}` } }}
                  onChange={(event: ChangeEvent<HTMLInputElement>) =>
                    onChange(withFolderAt(folders, index, { isRecursive: event.target.checked }))
                  }
                />
              }
              label={<Typography variant={"body2"}>{recursiveLabel}</Typography>}
            />

            <IconButton
              size={"small"}
              disabled={isDisabled}
              aria-label={`Remove ${folder.path || "the root"}`}
              onClick={() => onChange(withoutAt(folders, index))}
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
          onClick={() => onChange([...folders, { path: "", isRecursive: true }])}
        >
          {addLabel}
        </Button>
      </Box>
    </Stack>
  );
}
