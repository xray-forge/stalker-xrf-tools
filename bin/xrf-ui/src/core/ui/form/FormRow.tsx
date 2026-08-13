import { Box, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { Nullable } from "@/lib/types/general";

export interface IFormRowProps {
  label: string;
  description?: string;
  isRequired?: boolean;
  error?: Nullable<string>;
  /**
   * Puts the control beside the label instead of under it.
   */
  isInline?: boolean;
  children: ReactNode;
}

/**
 * One labelled row of a form.
 */
export function FormRow({ label, description, isRequired, error, isInline, children }: IFormRowProps): ReactElement {
  const heading: ReactElement = (
    <Box sx={{ minWidth: 0 }}>
      <Typography variant={"subtitle2"} component={"div"}>
        {label}
        {isRequired ? (
          <Typography component={"span"} sx={{ color: "error.main", marginLeft: 0.5 }} aria-hidden>
            *
          </Typography>
        ) : null}
      </Typography>

      {description ? (
        <Typography variant={"caption"} sx={{ display: "block", color: "text.secondary" }}>
          {description}
        </Typography>
      ) : null}
    </Box>
  );

  return (
    <Box
      sx={
        isInline
          ? { display: "flex", alignItems: "center", justifyContent: "space-between", gap: 2 }
          : { display: "flex", flexDirection: "column", gap: 0.75 }
      }
    >
      {heading}

      <Box sx={{ minWidth: 0, flexShrink: isInline ? 0 : undefined }}>{children}</Box>

      {error ? (
        <Typography variant={"caption"} sx={{ color: "error.main" }}>
          {error}
        </Typography>
      ) : null}
    </Box>
  );
}
