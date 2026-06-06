import { ContainerConfig } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";
import { ProjectManager } from "@/core/store/project";
import { ThemeManager } from "@/core/store/theme";

export function Application(): ReactElement {
  const config: ContainerConfig = useMemo(() => ({ bindings: [ProjectManager, ThemeManager] }), []);

  return (
    <ContainerProvider config={config}>
      <ApplicationProvider>
        <ApplicationRouter />
      </ApplicationProvider>
    </ContainerProvider>
  );
}
