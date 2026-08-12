import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { IEditorPanel, TEditorPanelSide } from "@/core/components/shell/EditorPanelsContext";
import { PanelStripeButton } from "@/core/components/shell/PanelStripeButton";
import { Nullable } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

export interface IApplicationPanelStripeProps {
  side: TEditorPanelSide;
  panels: Array<IEditorPanel>;
  activePanelId: Nullable<string>;
  /** Occupies the toolbar's band at the top: Home on the left, notifications on the right. */
  header?: ReactNode;
  /** Pinned to the bottom, below the application's panels. */
  footer?: ReactNode;
  onTogglePanel: (id: string) => void;
}

/**
 * One edge of the window frame: a fixed control at the top, the application's panels below it.
 *
 * Both sides render through here so they cannot drift apart. The stripe stays put even when an
 * application declares no panels, for the same reason every route has a toolbar - a frame that changes
 * shape as you move between applications is harder to read than one that does not.
 */
export function ApplicationPanelStripe({
  side,
  panels,
  activePanelId,
  header,
  footer,
  onTogglePanel,
}: IApplicationPanelStripeProps): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        width: LAYOUT.railWidth,
        minWidth: LAYOUT.railWidth,
        paddingBottom: 1,
        ...(side === "left" ? { borderRight: 1 } : { borderLeft: 1 }),
        borderColor: "divider",
        backgroundColor: "background.paper",
      }}
    >
      {/*
        Exactly the toolbar's height, so the control inside lines up with the title beside it rather
        than floating above it, and its border continues the toolbar's own.
      */}
      {header ? (
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            flexShrink: 0,
            width: "100%",
            height: LAYOUT.toolbarHeight,
            minHeight: LAYOUT.toolbarHeight,
            borderBottom: 1,
            borderColor: "divider",
          }}
        >
          {header}
        </Box>
      ) : null}

      <Box sx={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 0.5, paddingTop: 1 }}>
        {panels.map((panel: IEditorPanel) => (
          <PanelStripeButton
            key={panel.id}
            panel={panel}
            side={side}
            isActive={panel.id === activePanelId}
            onTogglePanel={onTogglePanel}
          />
        ))}
      </Box>

      <Box sx={{ flexGrow: 1 }} />

      {footer ? (
        <Box sx={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 0.5 }}>{footer}</Box>
      ) : null}
    </Box>
  );
}
