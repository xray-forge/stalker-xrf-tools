import { Box, Card, CardActionArea, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/applicationTools";

/**
 * Start page rendered inside the shell.
 *
 * A working surface rather than a launcher screen: the rail already navigates, so this exists to give
 * the window something useful at rest and to describe what each tool is for.
 *
 * It carries the same toolbar as every other route, including this one - otherwise entering a tool
 * from here shifted the content down by the toolbar's height. No back control, because home is where
 * back would go. The title falls back to the application name, since no tool owns `/`.
 */
export function Root(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <EditorLayout toolbar={<EditorToolbar />}>
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Typography variant={"body2"} sx={{ color: "text.secondary", marginBottom: 3 }}>
          Inspect and edit S.T.A.L.K.E.R. gamedata. Pick a tool from the rail, or start below.
        </Typography>

        <Box sx={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(240px, 1fr))", gap: 2 }}>
          {APPLICATION_TOOLS.map((tool: IApplicationTool) => (
            <Card key={tool.path}>
              <CardActionArea sx={{ padding: 2 }} onClick={() => navigate(tool.path, { replace: true })}>
                <Box sx={{ display: "flex", alignItems: "center", gap: 1, marginBottom: 0.5 }}>
                  <Box sx={{ display: "flex", color: "primary.main" }}>{tool.icon}</Box>
                  <Typography variant={"subtitle2"}>{tool.label}</Typography>
                </Box>

                <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
                  {tool.description}
                </Typography>
              </CardActionArea>
            </Card>
          ))}
        </Box>
      </Box>
    </EditorLayout>
  );
}
