import { Typography } from "@mui/material";
import { ReactElement } from "react";

export function EditorToolbarPathSeparator(): ReactElement {
  return (
    <Typography aria-hidden={true} variant={"body2"} sx={{ color: "text.disabled", userSelect: "none" }}>
      ›
    </Typography>
  );
}
