import { default as ImageIcon } from "@mui/icons-material/Image";
import { lazy } from "react";

import { AssetService } from "@/core/assets/services";
import { EquipmentService } from "@/core/equipment-icons";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EQUIPMENT_ICONS_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, EquipmentService] },
  Component: lazy(() =>
    import("@/applications/equipment-icons").then((it) => ({
      default: it.EquipmentIconsApplication,
    }))
  ),
  preload: () => import("@/applications/equipment-icons"),
  description: "Inspect and edit equipment inventory icons",
  group: EApplicationGroupId.ICONS,
  icon: <ImageIcon />,
  id: EApplicationId.EQUIPMENT_ICONS,
  label: "Equipment icons",
  path: "/equipment-icons",
  status: EApplicationStatus.READY,
};
