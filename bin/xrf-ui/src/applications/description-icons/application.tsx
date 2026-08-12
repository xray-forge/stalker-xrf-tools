import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";

export const DESCRIPTION_ICONS_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/description-icons").then((it) => ({
      default: it.DescriptionIconsApplication,
    }))
  ),
  description: "Inspect and edit item description icons",
  group: EApplicationGroupId.ICONS,
  icon: <DescriptionIcon />,
  id: EApplicationId.DESCRIPTION_ICONS,
  label: "Description icons",
  path: "/description-icons",
  status: EApplicationStatus.PLANNED,
};
