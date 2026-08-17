import { Box, Divider, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface IVisualPanelSectionProps {
  title: ReactNode;
  /** What distinguishes this group from a similar one beside it. */
  caption?: ReactNode;
  children: ReactNode;
  /** Suppresses the leading divider, so the first group does not draw one against the panel title. */
  isFirst?: boolean;
}

/**
 * A titled group of rows.
 *
 * Panels are read by eye rather than searched, so facts are grouped under a heading instead of running together as one
 * list. The heading carries the group's own subject, which is what makes two similar groups - declared and measured
 * bounds, one submesh and the next - tellable apart at a glance.
 */
export function VisualPanelSection({ title, caption, children, isFirst }: IVisualPanelSectionProps): ReactElement {
  return (
    <Box sx={{ paddingX: 2, paddingTop: isFirst ? 2 : 1.5, paddingBottom: 1.5 }}>
      {isFirst ? null : <Divider sx={{ marginBottom: 1.5, marginX: -2 }} />}

      <Typography
        variant={"subtitle2"}
        sx={{ fontSize: "0.75rem", letterSpacing: "0.08em", textTransform: "uppercase", color: "text.secondary" }}
      >
        {title}
      </Typography>

      {caption ? (
        <Typography variant={"caption"} sx={{ display: "block", color: "text.disabled", wordBreak: "break-all" }}>
          {caption}
        </Typography>
      ) : null}

      <Box sx={{ marginTop: 1 }}>{children}</Box>
    </Box>
  );
}
