import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface IEditorLayoutProps {
  toolbar?: ReactNode;
  menu?: ReactNode;
  aside?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

/**
 * Workspace shell shared by every editor.
 *
 * Toolbar on top, footer at the bottom, and a middle row of optional left menu, content, optional right
 * aside. Content gets `minWidth: 0` and `minHeight: 0` so tables, canvases and trees scroll inside their
 * own region instead of pushing the page out.
 */
export function EditorLayout({ toolbar, menu, aside, footer, children }: IEditorLayoutProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      {toolbar}

      <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
        {menu}

        <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>{children}</Box>

        {aside}
      </Box>

      {footer}
    </Box>
  );
}
