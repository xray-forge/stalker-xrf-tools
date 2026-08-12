import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface IEditorLayoutProps {
  toolbar?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

/**
 * Workspace shell shared by every application.
 */
export function EditorLayout({ toolbar, footer, children }: IEditorLayoutProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      {toolbar}

      <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>{children}</Box>

      {footer}
    </Box>
  );
}
