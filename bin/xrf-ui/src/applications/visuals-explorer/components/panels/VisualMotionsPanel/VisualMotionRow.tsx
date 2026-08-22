import { Box, Chip, Typography } from "@mui/material";
import { ReactElement } from "react";

import { VisualMotionDependency } from "@/core/bindings/types/xrf-visual";
import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IVisualMotionRowProps extends BaseComponentProps {
  motion: VisualMotionDependency;
}

/**
 * One motion reference and whether it was found.
 *
 * A reference may be a mask naming a set, so a located outcome reports how many files answered it rather than implying
 * one: `wpn\wpn_ak74_*.omf` resolving to two files and to twenty are different situations.
 */
export function VisualMotionRow({
  "data-testid": dataTestId = "visual-motion-row",
  id,
  className,
  motion,
}: IVisualMotionRowProps): ReactElement {
  const { resolution } = motion;
  const located: number = resolution.kind === "resolved" ? resolution.assets.length : 0;

  return (
    <Box
      data-testid={dataTestId}
      id={id}
      className={className}
      sx={{ display: "flex", alignItems: "baseline", justifyContent: "space-between", gap: 1, paddingY: 0.4 }}
    >
      <Typography variant={"body2"} sx={{ minWidth: 0, wordBreak: "break-all" }}>
        {motion.reference}
      </Typography>

      {located > 0 ? (
        <Chip
          size={"small"}
          color={"success"}
          variant={"outlined"}
          label={located > 1 ? `${located} files` : "Found"}
          sx={{ flexShrink: 0 }}
        />
      ) : (
        <Chip
          size={"small"}
          color={resolution.kind === "rejected" ? "error" : "warning"}
          variant={"outlined"}
          label={resolution.kind === "rejected" ? "Unusable" : "Not found"}
          sx={{ flexShrink: 0 }}
        />
      )}
    </Box>
  );
}
