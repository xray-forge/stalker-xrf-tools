import { default as HelpIcon } from "@mui/icons-material/Help";
import { Box, Button, Card, CardActionArea, Typography } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/applicationTools";

const HELP_LINK: string = "https://xray-forge.github.io/stalker-xrf-book/tools/app/app.html";

/**
 * Start page rendered inside the shell.
 *
 * A working surface rather than a launcher screen: the rail already navigates, so this exists to give
 * the window something useful at rest and to describe what each tool is for.
 */
export function Root(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onOpenHelp = useCallback(() => {
    open(HELP_LINK).catch(console.error);
  }, []);

  return (
    <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
      <Box sx={{ display: "flex", alignItems: "center", gap: 1, marginBottom: 0.5 }}>
        <Typography variant={"h5"}>XRF development tools</Typography>

        <Button size={"small"} startIcon={<HelpIcon />} onClick={onOpenHelp}>
          Documentation
        </Button>
      </Box>

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
  );
}
