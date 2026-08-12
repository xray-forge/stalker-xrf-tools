import { Box, Card, CardActionArea, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";
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
  items: Array<IToolNavigatorItem>;
}

/**
 * Landing pane for one editor, listing the things that editor can do.
 */
export function ToolNavigator({ items }: IToolNavigatorProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <EditorLayout
      toolbar={
        // A tool landing pane is one level down from home, so it offers the same way back as the
        // editors inside it do.
        <EditorToolbar backPath={"/"} />
      }
    >
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Box sx={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(240px, 1fr))", gap: 2 }}>
          {items.map((item: IToolNavigatorItem) => (
            <Card key={item.to + item.label} sx={{ display: "flex", flexDirection: "column" }}>
              <CardActionArea
                sx={{ flexGrow: 1, padding: 2 }}
                onClick={() => navigate(item.to, { replace: true })}
              >
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
