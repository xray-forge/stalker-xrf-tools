import { ContainerConfig } from "@wirestate/core";
import { DevToolsPlugin } from "@wirestate/core/devtools";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";
import { ProjectService } from "@/core/store/project";

export function Application(): ReactElement {
  const config: ContainerConfig = useMemo(
    () => ({ bindings: [ProjectService], plugins: [new DevToolsPlugin({ label: "xrf-tools" })] }),
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
