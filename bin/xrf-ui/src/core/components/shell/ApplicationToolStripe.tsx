import { Box, IconButton, Tooltip } from "@mui/material";
import { ReactElement } from "react";

import { IEditorTool } from "@/core/components/shell/EditorToolsContext";
import { Nullable } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

export interface IApplicationToolStripeProps {
  tools: Array<IEditorTool>;
  globalTools: Array<IEditorTool>;
  activeToolId: Nullable<string>;
  onToggleTool: (id: string) => void;
}

interface IToolStripeButtonProps {
  tool: IEditorTool;
  isActive: boolean;
  onToggleTool: (id: string) => void;
}

function ToolStripeButton({ tool, isActive, onToggleTool }: IToolStripeButtonProps): ReactElement {
  return (
    <Tooltip title={tool.label} placement={"left"}>
      <IconButton
        aria-label={tool.label}
        aria-pressed={isActive}
        sx={{
          width: 36,
          height: 36,
          borderRadius: 1,
          color: isActive ? "primary.main" : "text.secondary",
          backgroundColor: isActive ? "action.selected" : "transparent",
        }}
        onClick={() => onToggleTool(tool.id)}
      >
        {tool.icon}
      </IconButton>
    </Tooltip>
  );
}

/**
 * The right hand tool stripe, mirroring the application rail on the left.
 */
export function ApplicationToolStripe({
  tools,
  globalTools,
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
        <ToolStripeButton key={tool.id} tool={tool} isActive={tool.id === activeToolId} onToggleTool={onToggleTool} />
      ))}

      <Box sx={{ flexGrow: 1 }} />

      {globalTools.length ? <Box sx={{ width: 24, borderBottom: 1, borderColor: "divider", marginY: 0.5 }} /> : null}

      {globalTools.map((tool: IEditorTool) => (
        <ToolStripeButton key={tool.id} tool={tool} isActive={tool.id === activeToolId} onToggleTool={onToggleTool} />
      ))}
    </Box>
  );
}
