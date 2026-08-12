import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";
import { NotificationsService } from "@/core/store/notifications";
import { ProjectService } from "@/core/store/project";

/**
 * The root container.
 */
export function Application(): ReactElement {
  const config: ContainerConfig = useMemo(
    () => ({
      bindings: [ProjectService, NotificationsService],
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
