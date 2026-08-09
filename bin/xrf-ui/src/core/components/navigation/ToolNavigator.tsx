import { default as HelpIcon } from "@mui/icons-material/Help";
import { Box, Button, Card, CardActionArea, Typography } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export interface IToolNavigatorItem {
  label: string;
  description?: string;
  icon?: ReactNode;
  to: string;
  isSecondary?: boolean;
}

export interface IToolNavigatorProps {
  title: string;
  helpLink: string;
  items: Array<IToolNavigatorItem>;
}

/**
 * Landing pane for one editor, listing the things that editor can do.
 *
 * Fills the shell's content area rather than centering a narrow card in it: a card floating in an
 * otherwise empty window is the single most phone-like thing this app used to do.
 */
export function ToolNavigator({ title, helpLink, items }: IToolNavigatorProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onOpenHelp = useCallback(() => {
    open(helpLink).catch(console.error);
  }, [helpLink]);

  const primaryItems: Array<IToolNavigatorItem> = items.filter((item) => !item.isSecondary);

  return (
    <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
      <Box sx={{ display: "flex", alignItems: "center", gap: 1, marginBottom: 3 }}>
        <Typography variant={"h5"}>{title}</Typography>

        <Button size={"small"} startIcon={<HelpIcon />} onClick={onOpenHelp}>
          Documentation
        </Button>
      </Box>

      <Box sx={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(240px, 1fr))", gap: 2 }}>
        {primaryItems.map((item: IToolNavigatorItem) => (
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
  );
}
