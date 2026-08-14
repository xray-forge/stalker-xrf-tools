import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EQUIPMENT_ICONS_UNPACKER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./EquipmentIconsUnpackerApplication").then((it) => ({ default: it.EquipmentIconsUnpackerApplication }))
  ),
  preload: () => import("./EquipmentIconsUnpackerApplication"),
  description: "Extract individual icons from an equipment sprite",
  group: EApplicationGroupId.ICONS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.EQUIPMENT_ICONS_UNPACKER,
  label: "Equipment icons unpacker",
  path: "/equipment-icons-unpacker",
  status: EApplicationStatus.PLANNED,
};
