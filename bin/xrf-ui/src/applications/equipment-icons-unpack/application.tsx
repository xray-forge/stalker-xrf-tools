import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const EQUIPMENT_ICONS_UNPACK_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/equipment-icons-unpack").then((it) => ({
      default: it.EquipmentIconsUnpackApplication,
    }))
  ),
  description: "Extract individual icons from an equipment sprite",
  group: EApplicationGroupId.ICONS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.EQUIPMENT_ICONS_UNPACK,
  label: "Equipment unpack",
  path: "/equipment-icons-unpack",
  status: EApplicationStatus.PLANNED,
};
