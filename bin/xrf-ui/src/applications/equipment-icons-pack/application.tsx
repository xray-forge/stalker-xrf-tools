import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const EQUIPMENT_ICONS_PACK_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/equipment-icons-pack").then((it) => ({
      default: it.EquipmentIconsPackApplication,
    }))
  ),
  description: "Build an equipment sprite from individual icons",
  group: EApplicationGroupId.ICONS,
  icon: <Inventory2Icon />,
  id: EApplicationId.EQUIPMENT_ICONS_PACK,
  label: "Equipment pack",
  path: "/equipment-icons-pack",
  status: EApplicationStatus.READY,
};
