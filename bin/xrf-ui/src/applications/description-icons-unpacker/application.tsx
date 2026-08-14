import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const DESCRIPTION_ICONS_UNPACKER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./DescriptionIconsUnpackerApplication").then((it) => ({ default: it.DescriptionIconsUnpackerApplication }))
  ),
  preload: () => import("./DescriptionIconsUnpackerApplication"),
  description: "Extract individual icons from a description sprite",
  group: EApplicationGroupId.ICONS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.DESCRIPTION_ICONS_UNPACKER,
  label: "Description icons unpacker",
  path: "/description-icons-unpacker",
  status: EApplicationStatus.PLANNED,
};
