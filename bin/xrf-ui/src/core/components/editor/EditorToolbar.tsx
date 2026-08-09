import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { AppBar, Box, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export interface IEditorToolbarProps {
  title: string;
  subtitle?: ReactNode;
  backPath?: string;
  actions?: ReactNode;
}

/**
 * Header bar shared by every editor workspace.
 *
 * Fixes where the three moving parts live: back on the left, what is open in the middle, tools on the
 * right. Back returns to the editor's own navigator and does not touch open state - closing a file is a
 * separate action and belongs in the side menu.
 */
export function EditorToolbar({ title, subtitle, backPath, actions }: IEditorToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onBack = useCallback(() => {
    if (backPath) {
      navigate(backPath, { replace: true });
    }
  }, [navigate, backPath]);

  return (
    <AppBar position={"relative"} sx={{ flexShrink: 0 }}>
      <Toolbar variant={"dense"}>
        {backPath ? (
          <Tooltip title={"Back"}>
            <IconButton edge={"start"} color={"inherit"} sx={{ marginRight: 1 }} onClick={onBack}>
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
