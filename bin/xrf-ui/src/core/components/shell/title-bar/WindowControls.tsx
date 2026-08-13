import { Box } from "@mui/material";
import { ReactElement } from "react";

import { CloseGlyph, MaximizeGlyph, MinimizeGlyph, RestoreGlyph } from "@/core/components/shell/title-bar/CaptionGlyph";
import { WindowControlButton } from "@/core/components/shell/title-bar/WindowControlButton";
import { IWindowControls, useWindowControls } from "@/lib/tauri/use-window-controls";
import { Nullable } from "@/lib/types/general";

/**
 * The minimize, maximize and close group, in the order the system puts them.
 *
 * Owns the window handle so nothing above it has to know whether one exists. Outside a tauri webview
 * the group renders nothing at all rather than rendering inert buttons, because a close button that
 * does nothing is worse than no close button.
 */
export function WindowControls(): Nullable<ReactElement> {
  const { isAvailable, isMaximized, minimize, toggleMaximize, close }: IWindowControls = useWindowControls();

  return isAvailable ? (
    <Box sx={{ display: "flex", alignItems: "center", flexShrink: 0 }}>
      <WindowControlButton label={"Minimize"} onClick={minimize}>
        <MinimizeGlyph />
      </WindowControlButton>

      <WindowControlButton label={isMaximized ? "Restore down" : "Maximize"} onClick={toggleMaximize}>
        {isMaximized ? <RestoreGlyph /> : <MaximizeGlyph />}
      </WindowControlButton>

      <WindowControlButton isDanger={true} label={"Close"} onClick={close}>
        <CloseGlyph />
      </WindowControlButton>
    </Box>
  ) : null;
}
