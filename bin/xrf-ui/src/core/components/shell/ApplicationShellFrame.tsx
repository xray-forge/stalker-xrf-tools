import { Box } from "@mui/material";
import { ReactElement, ReactNode, useCallback, useState } from "react";
import { useLocation } from "react-router-dom";

import { ApplicationCrash } from "@/core/components/error/ApplicationCrash";
import { ErrorBoundary, IErrorBoundaryFallbackProps } from "@/core/components/error/ErrorBoundary";
import { ApplicationScope } from "@/core/components/shell/ApplicationScope";
import { ApplicationStatusBar } from "@/core/components/shell/footer/ApplicationStatusBar";
import { EditorToolbarHostContext } from "@/core/components/shell/header/editor-toolbar-host";
import { ApplicationPanelSlot } from "@/core/components/shell/panel/ApplicationPanelSlot";
import { ApplicationPanelStripe } from "@/core/components/shell/panel/ApplicationPanelStripe";
import { ApplicationRail } from "@/core/components/shell/panel/ApplicationRail";
import {
  IEditorPanel,
  selectPanelsOnSide,
  useEditorPanelsRegistry,
} from "@/core/components/shell/panel/context";
import { NOTIFICATIONS_PANEL } from "@/core/components/shell/panel/global-panels";
import { PanelStripeButton } from "@/core/components/shell/panel/PanelStripeButton";
import { IPanelSlot, usePanelSlot } from "@/core/components/shell/panel/use-panel-slot";
import { ApplicationTitleBar } from "@/core/components/shell/title-bar/ApplicationTitleBar";
import { APPLICATION_SOURCE, IApplicationDescriptor } from "@/core/router/application";
import { findApplication } from "@/core/router/applications";
import { Nullable } from "@/core/types/general";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";

export interface IApplicationShellFrameProps {
  children: ReactNode;
}

/**
 * The window frame itself: rail and its panel on the left, panel and stripe on the right, status bar
 * along the bottom.
 *
 * Applications render into the middle only. Nothing inside can take the window over, which is what
 * separates a desktop tool from a stack of full screen pages.
 */
export function ApplicationShellFrame({ children }: IApplicationShellFrameProps): ReactElement {
  const panels: Array<IEditorPanel> = useEditorPanelsRegistry();
  const notify: TNotify = useNotify();

  const { pathname } = useLocation();

  // The element the routed content portals its toolbar into. Held here rather than in a provider of
  // its own: the frame hands it down and never reads it back.
  const [toolbarHost, setToolbarHost] = useState<Nullable<HTMLElement>>(null);

  const application: Nullable<IApplicationDescriptor> = findApplication(pathname);

  // Keyed per application so the visuals panel choice does not leak into another one.
  const applicationPath: string = application?.path ?? "root";

  const leftPanels: Array<IEditorPanel> = selectPanelsOnSide(panels, "left");
  const rightPanels: Array<IEditorPanel> = selectPanelsOnSide(panels, "right");

  const leftSlot: IPanelSlot = usePanelSlot("left", leftPanels, applicationPath);
  const rightSlot: IPanelSlot = usePanelSlot("right", rightPanels, applicationPath);

  const onError = useCallback((props: IErrorBoundaryFallbackProps) => <ApplicationCrash {...props} />, []);

  // Recorded as a real outcome rather than a dev trace: the user has already met the crash screen, and
  // this is what survives navigating away from it.
  const onCaught = useCallback(
    (error: Error, componentStack: Nullable<string>) =>
      notify({
        details: componentStack ? `${error.message}\n${componentStack}` : error.message,
        severity: ENotificationSeverity.ERROR,
        source: findApplication(pathname)?.id ?? APPLICATION_SOURCE,
        title: "The interface crashed and was replaced",
      }),
    [notify, pathname]
  );

  return (
    <EditorToolbarHostContext.Provider value={toolbarHost}>
      <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
        <ApplicationTitleBar toolbarRef={setToolbarHost} />

        <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
          <ApplicationRail
            panels={leftPanels}
            activePanelId={leftSlot.activePanelId}
            onTogglePanel={leftSlot.onTogglePanel}
          />

          <ApplicationScope key={applicationPath} application={application}>
            <ApplicationPanelSlot side={"left"} slot={leftSlot} />

            <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>
              <ErrorBoundary resetKey={pathname} fallback={onError} onCaught={onCaught}>
                {children}
              </ErrorBoundary>
            </Box>

            <ApplicationPanelSlot side={"right"} slot={rightSlot} />
          </ApplicationScope>

          <ApplicationPanelStripe
            side={"right"}
            panels={rightPanels}
            activePanelId={rightSlot.activePanelId}
            /* Below the application's own panels, mirroring the window controls in the opposite stripe. */
            footer={
              <PanelStripeButton
                panel={NOTIFICATIONS_PANEL}
                side={"right"}
                isActive={rightSlot.activePanelId === NOTIFICATIONS_PANEL.id}
                onTogglePanel={rightSlot.onTogglePanel}
              />
            }
            onTogglePanel={rightSlot.onTogglePanel}
          />
        </Box>

        <ApplicationStatusBar />
      </Box>
    </EditorToolbarHostContext.Provider>
  );
}
