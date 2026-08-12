import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";
import { ErrorCaptureService, NotificationsService } from "@/core/store/notifications";
import { ProjectService } from "@/core/store/project";
import { SettingsService } from "@/core/store/settings";

/**
 * The root container.
 */
export function Application(): ReactElement {
  const config: ContainerConfig = useMemo(
    () => ({
      bindings: [ProjectService, SettingsService, NotificationsService, ErrorCaptureService],
      plugins: [new EventsPlugin()],
    }),
    []
  );

  return (
    <ContainerProvider config={config}>
      <ApplicationProvider>
        <ApplicationRouter />
      </ApplicationProvider>
    </ContainerProvider>
  );
}
