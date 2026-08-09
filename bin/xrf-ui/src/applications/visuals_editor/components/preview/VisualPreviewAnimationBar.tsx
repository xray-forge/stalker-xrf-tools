import { default as PlayArrowIcon } from "@mui/icons-material/PlayArrow";
import { default as RepeatIcon } from "@mui/icons-material/Repeat";
import { Box, IconButton, MenuItem, Paper, Select, Slider, Tooltip, Typography } from "@mui/material";
import { ReactElement } from "react";

/**
 * Placeholder for the motion playback bar.
 *
 * Entirely disabled: omf motions are a later phase, and the bar exists now only so the preview page is
 * laid out around it rather than being rearranged once playback arrives.
 */
export function VisualPreviewAnimationBar(): ReactElement {
  return (
    <Paper
      square
      elevation={3}
      sx={{ display: "flex", alignItems: "center", gap: 1, paddingX: 1, paddingY: 0.5, flexShrink: 0 }}
    >
      <Select size={"small"} value={"none"} disabled sx={{ minWidth: 200 }}>
        <MenuItem value={"none"}>No motions loaded</MenuItem>
      </Select>

      <Tooltip title={"Playback (not implemented)"}>
        <span>
          <IconButton size={"small"} disabled>
            <PlayArrowIcon />
          </IconButton>
        </span>
      </Tooltip>

      <Slider size={"small"} value={0} disabled sx={{ marginX: 1 }} />

      <Typography variant={"caption"} sx={{ minWidth: 72, textAlign: "right" }}>
        0 / 0
      </Typography>

      <Tooltip title={"Loop (not implemented)"}>
        <span>
          <IconButton size={"small"} disabled>
            <RepeatIcon />
          </IconButton>
        </span>
      </Tooltip>

      <Box sx={{ width: 8 }} />
    </Paper>
  );
}
