import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { AppBar, Box, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { findApplicationTool, IApplicationTool } from "@/core/components/shell/applicationTools";
import { Optional } from "@/core/types/general";

export interface IEditorToolbarProps {
  /** Overrides the tool name resolved from the route. Rarely needed. */
  title?: string;
  /** What is open, when the editor knows. Counts and state belong in the status bar instead. */
  subtitle?: ReactNode;
  backPath?: string;
  /**
   * Runs instead of plain navigation when leaving.
   */
  onBack?: () => void;
  actions?: ReactNode;
}

/**
 * Header bar shared by every editor surface, including the landing panes.
 */
export function EditorToolbar({ title, subtitle, backPath, onBack, actions }: IEditorToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const tool: Optional<IApplicationTool> = findApplicationTool(pathname);

  const onLeave = useCallback(() => {
    if (onBack) {
      onBack();
    } else if (backPath) {
      navigate(backPath, { replace: true });
    }
  }, [navigate, backPath, onBack]);

  return (
    <AppBar position={"relative"} sx={{ flexShrink: 0 }}>
      <Toolbar variant={"dense"}>
        {backPath || onBack ? (
          <Tooltip title={onBack ? "Close and go back" : "Back"}>
            <IconButton edge={"start"} color={"inherit"} sx={{ marginRight: 1 }} onClick={onLeave}>
              <ArrowBackIcon />
            </IconButton>
          </Tooltip>
        ) : null}

        <Typography variant={"h6"} component={"div"} noWrap>
          {title ?? tool?.title ?? "XRF tools"}
        </Typography>

        {subtitle ? (
          <Typography variant={"body2"} component={"div"} noWrap sx={{ marginLeft: 1.5, opacity: 0.7, minWidth: 0 }}>
            {subtitle}
          </Typography>
        ) : null}

        <Box sx={{ flexGrow: 1 }} />

        {actions}
      </Toolbar>
    </AppBar>
  );
}
