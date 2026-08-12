import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ExportsApplicationScreen } from "@/applications/exports/ExportsApplicationScreen";
import { ExportsService } from "@/applications/exports/store/exports";

/**
 * Inspect the typescript extern declarations an XRF project exports.
 */
export function ExportsApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [ExportsService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <ExportsApplicationScreen />
    </ContainerProvider>
  );
}
