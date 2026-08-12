import { Box } from "@mui/material";
import { ReactElement } from "react";

import { TEditorPanelSide } from "@/core/components/shell/EditorPanelsContext";
import { PanelResizer } from "@/core/components/shell/PanelResizer";
import { IPanelSlot } from "@/core/components/shell/use-panel-slot";

export interface IApplicationPanelSlotProps {
  side: TEditorPanelSide;
  slot: IPanelSlot;
}

/**
 * The docked panel on one side of the content, and the handle that sizes it.
 *
 * Two boxes rather than one: the scrolling half has to be a separate child, or the resizer - absolutely
 * positioned to cost no width - would scroll away with the panel's content.
 */
export function ApplicationPanelSlot({ side, slot }: IApplicationPanelSlotProps): ReactElement | null {
  if (!slot.activePanel) {
    return null;
  }

  return (
    <Box
      sx={{
        position: "relative",
        display: "flex",
        flexDirection: "column",
        width: slot.width,
        minWidth: slot.width,
        minHeight: 0,
        ...(side === "left" ? { borderRight: 1 } : { borderLeft: 1 }),
        borderColor: "divider",
        backgroundColor: "background.default",
      }}
    >
      <Box sx={{ flexGrow: 1, minHeight: 0, overflowY: "auto" }}>{slot.activePanel.render()}</Box>

      <PanelResizer side={side} width={slot.width} onResize={slot.onResize} />
    </Box>
  );
}
