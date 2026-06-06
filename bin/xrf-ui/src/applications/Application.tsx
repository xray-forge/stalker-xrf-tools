import { ContainerConfig } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";
import { ProjectService } from "@/core/store/project";
import { ThemeService } from "@/core/store/theme";

export function Application(): ReactElement {
  const config: ContainerConfig = useMemo(() => ({ bindings: [ThemeService, ProjectService] }), []);

  return (
    <ContainerProvider config={config}>
      <ApplicationProvider>
        <ApplicationRouter />
      </ApplicationProvider>
    </ContainerProvider>
  );
}
