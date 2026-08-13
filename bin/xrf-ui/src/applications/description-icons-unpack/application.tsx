import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const DESCRIPTION_ICONS_UNPACK_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/description-icons-unpack").then((it) => ({
      default: it.DescriptionIconsUnpackApplication,
    }))
  ),
  preload: () => import("@/applications/description-icons-unpack"),
  description: "Extract individual icons from a description sprite",
  group: EApplicationGroupId.ICONS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.DESCRIPTION_ICONS_UNPACK,
  label: "Description unpack",
  path: "/description-icons-unpack",
  status: EApplicationStatus.PLANNED,
};
