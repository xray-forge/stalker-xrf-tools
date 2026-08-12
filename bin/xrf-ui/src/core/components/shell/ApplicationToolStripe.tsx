import { Box } from "@mui/material";
import { ReactElement } from "react";

import { IEditorTool } from "@/core/components/shell/EditorToolsContext";
import { ToolStripeButton } from "@/core/components/shell/ToolStripeButton";
import { Nullable } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

export interface IApplicationToolStripeProps {
  tools: Array<IEditorTool>;
  globalTools: Array<IEditorTool>;
  activeToolId: Nullable<string>;
  onToggleTool: (id: string) => void;
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
