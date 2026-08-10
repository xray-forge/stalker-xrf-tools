import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { AppBar, Box, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { findApplicationTool, IApplicationTool } from "@/core/components/shell/applicationTools";
import { Optional } from "@/core/types/general";

export interface IEditorToolbarProps {
  /** Keeps the leaving control in place but inert, rather than removing it mid-operation. */
  isBackDisabled?: boolean;
  /** Overrides the tool name resolved from the route. Rarely needed. */
  title?: string;
  /** What is open, when the editor knows. Counts and state belong in the status bar instead. */
  subtitle?: ReactNode;
  backPath?: string;
  actions?: ReactNode;
  /** Runs instead of plain navigation when leaving. */
  onBack?: () => void;
}

/**
 * Header bar shared by every editor surface, including the landing panes.
 */
export function EditorToolbar({
  isBackDisabled,
  title,
  subtitle,
  backPath,
  actions,
  onBack,
}: IEditorToolbarProps): ReactElement {
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
          // A disabled button cannot receive the tooltip's events, so it is wrapped in a span. That
          // wrapper is why the accessible name is set on the button itself and the tooltip is marked
          // `describeChild`: left to its default the tooltip would label the span as well, giving the
          // control two names.
          <Tooltip describeChild title={onBack ? "Close and go back" : "Back"}>
            <span>
              <IconButton
                edge={"start"}
                color={"inherit"}
                aria-label={onBack ? "Close and go back" : "Back"}
                disabled={isBackDisabled}
                sx={{ marginRight: 1 }}
                onClick={onLeave}
              >
                <ArrowBackIcon />
              </IconButton>
            </span>
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
