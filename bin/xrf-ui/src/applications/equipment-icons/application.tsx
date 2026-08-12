import { default as ImageIcon } from "@mui/icons-material/Image";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const EQUIPMENT_ICONS_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/equipment-icons").then((it) => ({
      default: it.EquipmentIconsApplication,
    }))
  ),
  description: "Inspect and edit equipment inventory icons",
  group: EApplicationGroupId.ICONS,
  icon: <ImageIcon />,
  id: EApplicationId.EQUIPMENT_ICONS,
  label: "Equipment icons",
  path: "/equipment-icons",
  status: EApplicationStatus.READY,
};
