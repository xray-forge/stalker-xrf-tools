import { Typography } from "@mui/material";
import { ReactElement } from "react";

interface IPackerPathTextProps {
  value: string;
}

/**
 * A filesystem path, shown in full.
 *
 * Paths are long and their tail is the part that identifies them, so they wrap rather than truncate.
 */
export function PackerPathText({ value }: IPackerPathTextProps): ReactElement {
  return (
    <Typography variant={"body2"} className={"monospace"} sx={{ wordBreak: "break-all" }}>
      {value}
    </Typography>
  );
}
