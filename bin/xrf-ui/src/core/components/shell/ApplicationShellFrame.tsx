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
import { IEditorPanel, selectPanelsOnSide, useEditorPanelsRegistry } from "@/core/components/shell/panel/context";
import { NOTIFICATIONS_PANEL } from "@/core/components/shell/panel/notifications/notification-panel";
import { PanelStripeButton } from "@/core/components/shell/panel/PanelStripeButton";
import { IPanelSlot, usePanelSlot } from "@/core/components/shell/panel/use-panel-slot";
import { ApplicationTitleBar } from "@/core/components/shell/title-bar/ApplicationTitleBar";
import { APPLICATION_SOURCE, IApplicationDescriptor } from "@/core/router/application";
import { findApplication } from "@/core/router/applications";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";

export interface IApplicationShellFrameProps extends BaseComponentProps {
  children: ReactNode;
}

/**
 * The window frame itself: rail and its panel on the left, panel and stripe on the right, status bar
 * along the bottom.
 */
export function ApplicationShellFrame({
  "data-testid": dataTestId = "application-shell-frame",
  id = "application-shell-frame",
  className,
  children,
}: IApplicationShellFrameProps): ReactElement {
  const notify: TNotify = useNotify();
  const panels: ReadonlyArray<IEditorPanel> = useEditorPanelsRegistry();

  const { pathname } = useLocation();

  // The element the routed content portals its toolbar into. Held here rather than in a provider of
  // its own: the frame hands it down and never reads it back.
  const [toolbarHost, setToolbarHost] = useState<Nullable<HTMLElement>>(null);

  const application: Nullable<IApplicationDescriptor> = findApplication(pathname);
  const applicationPath: string = application?.path ?? "root";

  const leftPanels: Array<IEditorPanel> = selectPanelsOnSide(panels, "left");
  const applicationRightPanels: Array<IEditorPanel> = selectPanelsOnSide(panels, "right");
  const rightPanels: Array<IEditorPanel> = [...applicationRightPanels, NOTIFICATIONS_PANEL];

  const leftSlot: IPanelSlot = usePanelSlot("left", leftPanels, applicationPath);
  const rightSlot: IPanelSlot = usePanelSlot("right", rightPanels, "global");

  const onError = useCallback((props: IErrorBoundaryFallbackProps) => <ApplicationCrash {...props} />, []);

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
      <Box
        data-testid={dataTestId}
        id={id}
        className={className}
        sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}
      >
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
            panels={applicationRightPanels}
            activePanelId={rightSlot.activePanelId}
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
