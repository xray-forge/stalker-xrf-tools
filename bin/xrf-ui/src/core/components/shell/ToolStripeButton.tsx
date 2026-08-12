import { IconButton, Tooltip } from "@mui/material";
import { ReactElement } from "react";

import { IEditorTool } from "@/core/components/shell/EditorToolsContext";

export interface IToolStripeButtonProps {
  tool: IEditorTool;
  isActive: boolean;
  onToggleTool: (id: string) => void;
}

/**
 * One control in the tool stripe.
 *
 * Selects and toggles in the same click: pressing the open one closes it.
 */
export function ToolStripeButton({ tool, isActive, onToggleTool }: IToolStripeButtonProps): ReactElement {
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
