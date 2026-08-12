import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { EquipmentIconsApplicationScreen } from "@/applications/equipment-icons/EquipmentIconsApplicationScreen";
import { AssetService } from "@/lib/assets";
import { EquipmentService } from "@/lib/icons";

/**
 * Inspect and edit the equipment inventory icon sprite.
 */
export function EquipmentIconsApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [AssetService, EquipmentService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <EquipmentIconsApplicationScreen />
    </ContainerProvider>
  );
}
