import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { AssetService } from "@/lib/xrf/assets";
import { EquipmentService } from "@/lib/xrf/icons";

export const EQUIPMENT_ICONS_PACK_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, EquipmentService] },
  Component: lazy(() =>
    import("@/applications/equipment-icons-pack").then((it) => ({
      default: it.EquipmentIconsPackApplication,
    }))
  ),
  preload: () => import("@/applications/equipment-icons-pack"),
  description: "Build an equipment sprite from individual icons",
  group: EApplicationGroupId.ICONS,
  icon: <Inventory2Icon />,
  id: EApplicationId.EQUIPMENT_ICONS_PACK,
  label: "Equipment pack",
  path: "/equipment-icons-pack",
  status: EApplicationStatus.READY,
};
