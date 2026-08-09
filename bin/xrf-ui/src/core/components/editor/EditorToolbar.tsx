import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { AppBar, Box, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export interface IEditorToolbarProps {
  title: string;
  subtitle?: ReactNode;
  backPath?: string;
  actions?: ReactNode;
  /**
   * Runs instead of plain navigation when leaving.
   */
  onBack?: () => void;
}

/**
 * Header bar shared by every editor workspace.
 *
 * Fixes where the three moving parts live: leaving on the left, what is open in the middle, tools on
 * the right.
 */
export function EditorToolbar({ title, subtitle, backPath, onBack, actions }: IEditorToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

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
          {title}
        </Typography>

        {subtitle ? (
          <Typography variant={"body2"} component={"div"} noWrap sx={{ marginLeft: 1.5, opacity: 0.7 }}>
            {subtitle}
          </Typography>
        ) : null}

        <Box sx={{ flexGrow: 1 }} />

        {actions}
      </Toolbar>
    </AppBar>
  );
}
