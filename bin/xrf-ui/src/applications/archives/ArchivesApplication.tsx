import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { ArchivesApplicationScreen } from "@/applications/archives/ArchivesApplicationScreen";
import { ArchivesService } from "@/applications/archives/store/archives";
import { AssetService } from "@/lib/assets";

/**
 * Browse the contents of game archives.
 *
 * Declares its own container, like every application does: the services live and die with this screen,
 * so nothing another application runs can close what this one has open.
 */
export function ArchivesApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [AssetService, ArchivesService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <ArchivesApplicationScreen />
    </ContainerProvider>
  );
}
