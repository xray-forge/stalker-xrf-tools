import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { default as FileOpenIcon } from "@mui/icons-material/FileOpen";
import { default as SaveAltIcon } from "@mui/icons-material/SaveAlt";
import { Button, IconButton, Stack, Tooltip } from "@mui/material";
import { ReactElement } from "react";

interface IPackerToolbarActionsProps {
  isBusy: boolean;
  isPackDisabled: boolean;
  onImport: () => void;
  onExport: () => void;
  onPack: () => void;
}

/**
 * Toolbar actions for the packer.
 *
 * The caption row is sized for icon buttons, so the two configuration actions are icons and only the
 * primary verb keeps its label. A tooltip carries what each icon means, and the disabled ones are
 * wrapped so the tooltip still reaches them.
 */
export function PackerToolbarActions({
  isBusy,
  isPackDisabled,
  onImport,
  onExport,
  onPack,
}: IPackerToolbarActionsProps): ReactElement {
  return (
    <Stack direction={"row"} spacing={0.5} sx={{ alignItems: "center", mr: 0.5 }}>
      <Tooltip describeChild title={"Import a packing configuration"}>
        <span>
          <IconButton aria-label={"Import packing configuration"} disabled={isBusy} onClick={onImport}>
            <FileOpenIcon />
          </IconButton>
        </span>
      </Tooltip>

      <Tooltip describeChild title={"Export these rules as a packing configuration"}>
        <span>
          <IconButton aria-label={"Export packing configuration"} disabled={isBusy} onClick={onExport}>
            <SaveAltIcon />
          </IconButton>
        </span>
      </Tooltip>

      <Tooltip describeChild title={isPackDisabled ? "Choose a source and an output first" : "Write the volumes"}>
        <span>
          <Button
            size={"small"}
            variant={"contained"}
            disabled={isPackDisabled}
            startIcon={<ArchiveIcon />}
            // Sized to the caption row rather than to a page button, which is what made it crowd the
            // separator beside it.
            sx={{
              height: 24,
              minWidth: 0,
              px: 1,
              fontSize: "0.75rem",
              lineHeight: 1,
              "& .MuiButton-startIcon": { mr: 0.5 },
            }}
            onClick={onPack}
          >
            Pack
          </Button>
        </span>
      </Tooltip>
    </Stack>
  );
}
