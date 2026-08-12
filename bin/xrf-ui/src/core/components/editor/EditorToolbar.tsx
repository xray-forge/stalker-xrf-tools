import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { AppBar, Box, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { IApplicationDescriptor } from "@/core/router/application";
import { findApplication } from "@/core/router/applications";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IEditorToolbarProps extends BaseComponentProps {
  /** Keeps the leaving control in place but inert, rather than removing it mid-operation. */
  isBackDisabled?: boolean;
  /** Overrides the application name resolved from the route. Rarely needed. */
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
  id = "editor-toolbar",
  isBackDisabled,
  title,
  subtitle,
  backPath,
  actions,
  onBack,
}: IEditorToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const application: Nullable<IApplicationDescriptor> = findApplication(pathname);

  const onLeave = useCallback(() => {
    if (onBack) {
      onBack();
    } else if (backPath) {
      navigate(backPath, { replace: true });
    }
  }, [navigate, backPath, onBack]);

  return (
    <AppBar id={id} position={"relative"} sx={{ flexShrink: 0 }}>
      <Toolbar variant={"dense"}>
        {backPath || onBack ? (
          // A disabled button cannot receive the tooltip's events, so it is wrapped in a span. That
          // wrapper is why the accessible name is set on the button itself and the tooltip is marked
          // `describeChild`: left to its default the tooltip would label the span as well, giving the
          // control two names.
          <Tooltip describeChild title={onBack ? "Close and go back" : "Back"}>
            <span>
              <IconButton
                color={"inherit"}
                aria-label={onBack ? "Close and go back" : "Back"}
                disabled={isBackDisabled}
                sx={{ marginRight: 0.5 }}
                onClick={onLeave}
              >
                <ArrowBackIcon fontSize={"small"} />
              </IconButton>
            </span>
          </Tooltip>
        ) : null}

        <Typography
          // Sized against the rail controls either side of it rather than as a page heading: they share
          // the toolbar's band, and an `h6` beside a 32px button read as two different registers.
          variant={"subtitle1"}
          component={"div"}
          noWrap={true}
          sx={{
            fontWeight: 600,
            marginLeft: backPath || onBack ? 0 : 2,
          }}
        >
          {title ?? application?.label ?? "Tools"}
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
