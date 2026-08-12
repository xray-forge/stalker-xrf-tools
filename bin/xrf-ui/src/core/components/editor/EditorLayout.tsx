import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";
import { createPortal } from "react-dom";

import { useEditorToolbarHost } from "@/core/components/shell/header/editor-toolbar-host";
import { Nullable } from "@/core/types/general";

export interface IEditorLayoutProps {
  toolbar?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

/**
 * Workspace shell shared by every application.
 */
export function EditorLayout({ toolbar, footer, children }: IEditorLayoutProps): ReactElement {
  const host: Nullable<HTMLElement> = useEditorToolbarHost();

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      {toolbar && host ? createPortal(toolbar, host) : toolbar}

      <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>{children}</Box>

      {footer}
    </Box>
  );
}
