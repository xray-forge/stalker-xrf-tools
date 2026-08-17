import { Typography } from "@mui/material";
import { ReactElement } from "react";

export interface IVisualMotionNamesProps {
  names: Array<string>;
}

/** A list of motion names, wrapped rather than truncated because the tail identifies them. */
export function VisualMotionNames({ names }: IVisualMotionNamesProps): ReactElement {
  return (
    <>
      {names.map((name) => (
        <Typography key={name} variant={"body2"} sx={{ paddingY: 0.4, lineHeight: 1.6, wordBreak: "break-all" }}>
          {name}
        </Typography>
      ))}
    </>
  );
}
