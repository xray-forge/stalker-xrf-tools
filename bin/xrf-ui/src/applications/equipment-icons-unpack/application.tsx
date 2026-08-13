import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EQUIPMENT_ICONS_UNPACK_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/equipment-icons-unpack").then((it) => ({
      default: it.EquipmentIconsUnpackApplication,
    }))
  ),
  preload: () => import("@/applications/equipment-icons-unpack"),
  description: "Extract individual icons from an equipment sprite",
  group: EApplicationGroupId.ICONS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.EQUIPMENT_ICONS_UNPACK,
  label: "Equipment unpack",
  path: "/equipment-icons-unpack",
  status: EApplicationStatus.PLANNED,
};
