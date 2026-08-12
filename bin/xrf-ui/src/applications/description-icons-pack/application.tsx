import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const DESCRIPTION_ICONS_PACK_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/description-icons-pack").then((it) => ({
      default: it.DescriptionIconsPackApplication,
    }))
  ),
  description: "Build a description sprite from individual icons",
  group: EApplicationGroupId.ICONS,
  icon: <Inventory2Icon />,
  id: EApplicationId.DESCRIPTION_ICONS_PACK,
  label: "Description pack",
  path: "/description-icons-pack",
  status: EApplicationStatus.PLANNED,
};
