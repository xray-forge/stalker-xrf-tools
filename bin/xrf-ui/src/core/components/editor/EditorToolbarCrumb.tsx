import { Tooltip, Typography } from "@mui/material";
import { ReactElement } from "react";

export interface IEditorToolbarCrumbProps {
  label: string;
  isDisabled?: boolean;
  /** Says what following the segment does, for a name the label alone cannot carry. */
  accessibleName?: string;
  /** Describes what following the segment does. Only meaningful when it is interactive. */
  hint?: string;
  onClick?: () => void;
}

/**
 * One segment of the caption's breadcrumb.
 */
export function EditorToolbarCrumb({
  isDisabled,
  label,
  accessibleName,
  hint,
  onClick,
}: IEditorToolbarCrumbProps): ReactElement {
  if (!onClick) {
    return (
      <Typography variant={"subtitle2"} noWrap={true} sx={{ fontWeight: 600, flexShrink: 0 }}>
        {label}
      </Typography>
    );
  }

  const crumb: ReactElement = (
    <Typography
      aria-label={accessibleName}
      component={"button"}
      disabled={isDisabled}
      variant={"subtitle2"}
      sx={{
        appearance: "none",
        background: "none",
        border: 0,
        padding: 0,
        cursor: isDisabled ? "default" : "pointer",
        color: isDisabled ? "text.disabled" : "text.secondary",
        fontWeight: 600,
        flexShrink: 0,
        "&:hover": {
          color: isDisabled ? "text.disabled" : "text.primary",
          textDecoration: isDisabled ? "none" : "underline",
        },
      }}
      onClick={onClick}
    >
      {label}
    </Typography>
  );

  return hint ? (
    <Tooltip describeChild title={hint}>
      <span>{crumb}</span>
    </Tooltip>
  ) : (
    crumb
  );
}
