import { Box, IconButton, Tooltip } from "@mui/material";
import { ReactElement } from "react";

import { IEditorTool } from "@/core/components/shell/EditorToolsContext";
import { Optional } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

export interface IApplicationToolStripeProps {
  tools: Array<IEditorTool>;
  activeToolId: Optional<string>;
  onToggleTool: (id: string) => void;
}

/**
 * The right hand tool stripe, mirroring the application rail on the left.
 *
 * Icons both select a panel and toggle it: clicking the open one closes it, which is what gives a 3D
 * viewport its full width back. It is part of the window frame and stays put even when the active
 * editor declares no tools, for the same reason every route now has a toolbar - a frame that changes
 * shape as you move between tools is harder to read than one that does not.
 */
export function ApplicationToolStripe({
  tools,
  activeToolId,
  onToggleTool,
}: IApplicationToolStripeProps): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 0.5,
        width: LAYOUT.railWidth,
        minWidth: LAYOUT.railWidth,
        paddingY: 1,
        borderLeft: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
      }}
    >
      {tools.map((tool: IEditorTool) => (
        <Tooltip key={tool.id} title={tool.label} placement={"left"}>
          <IconButton
            aria-label={tool.label}
            aria-pressed={tool.id === activeToolId}
            sx={{
              width: 36,
              height: 36,
              borderRadius: 1,
              color: tool.id === activeToolId ? "primary.main" : "text.secondary",
              backgroundColor: tool.id === activeToolId ? "action.selected" : "transparent",
            }}
            onClick={() => onToggleTool(tool.id)}
          >
            {tool.icon}
          </IconButton>
        </Tooltip>
      ))}
    </Box>
  );
}
