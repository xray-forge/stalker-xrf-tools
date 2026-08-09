import { default as HelpIcon } from "@mui/icons-material/Help";
import { Box, Card, CardActionArea, IconButton, Tooltip, Typography } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export interface IToolNavigatorItem {
  label: string;
  description?: string;
  icon?: ReactNode;
  to: string;
}

export interface IToolNavigatorProps {
  helpLink: string;
  items: Array<IToolNavigatorItem>;
}

/**
 * Landing pane for one editor, listing what that editor can do.
 *
 * Carries the same toolbar as the workspaces so the frame does not change shape when you move between
 * a tool's landing pane and its editors. The tool name comes from the route, not from the caller.
 */
export function ToolNavigator({ helpLink, items }: IToolNavigatorProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onOpenHelp = useCallback(() => {
    open(helpLink).catch(console.error);
  }, [helpLink]);

  return (
    <EditorLayout
      toolbar={
        <EditorToolbar
          actions={
            <Tooltip title={"Documentation"}>
              <IconButton color={"inherit"} onClick={onOpenHelp}>
                <HelpIcon />
              </IconButton>
            </Tooltip>
          }
        />
      }
    >
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Box sx={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(240px, 1fr))", gap: 2 }}>
          {items.map((item: IToolNavigatorItem) => (
            <Card key={item.to + item.label}>
              <CardActionArea sx={{ padding: 2 }} onClick={() => navigate(item.to, { replace: true })}>
                <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                  {item.icon ? <Box sx={{ display: "flex", color: "primary.main" }}>{item.icon}</Box> : null}
                  <Typography variant={"subtitle2"}>{item.label}</Typography>
                </Box>

                {item.description ? (
                  <Typography variant={"body2"} sx={{ color: "text.secondary", marginTop: 0.5 }}>
                    {item.description}
                  </Typography>
                ) : null}
              </CardActionArea>
            </Card>
          ))}
        </Box>
      </Box>
    </EditorLayout>
  );
}
