import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { SpawnApplicationScreen } from "@/applications/spawn/SpawnApplicationScreen";
import { SpawnFileService } from "@/lib/spawn-file";

/**
 * Browse and edit a packed spawn file.
 *
 * The pack and unpack applications bind their own `SpawnFileService`, which is the point: they used to
 * share this one and closed the file out from under an open editor on their way out.
 */
export function SpawnApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [SpawnFileService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <SpawnApplicationScreen />
    </ContainerProvider>
  );
}
