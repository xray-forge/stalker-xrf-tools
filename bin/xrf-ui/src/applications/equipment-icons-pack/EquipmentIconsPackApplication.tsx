import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { EquipmentIconsPackApplicationScreen } from "@/applications/equipment-icons-pack/EquipmentIconsPackApplicationScreen";
import { AssetService } from "@/lib/assets";
import { EquipmentService } from "@/lib/icons";

/**
 * Build an equipment sprite from a directory of individual icons.
 *
 * Binds its own `EquipmentService`: packing opens and closes files, and the equipment editor is a
 * separate application that must not notice.
 */
export function EquipmentIconsPackApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [AssetService, EquipmentService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <EquipmentIconsPackApplicationScreen />
    </ContainerProvider>
  );
}
