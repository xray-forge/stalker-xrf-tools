import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import { AssetService } from "@/core/assets/services";
import { EquipmentService } from "@/core/equipment-icons";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EQUIPMENT_ICONS_PACKER_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, EquipmentService] },
  Component: lazy(() =>
    import("./EquipmentIconsPackerApplication").then((it) => ({ default: it.EquipmentIconsPackerApplication }))
  ),
  preload: () => import("./EquipmentIconsPackerApplication"),
  description: "Build an equipment sprite from individual icons",
  group: EApplicationGroupId.ICONS,
  icon: <Inventory2Icon />,
  id: EApplicationId.EQUIPMENT_ICONS_PACKER,
  label: "Equipment icons packer",
  path: "/equipment-icons-packer",
  status: EApplicationStatus.READY,
};
