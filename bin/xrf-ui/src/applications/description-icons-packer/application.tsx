import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const DESCRIPTION_ICONS_PACKER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./DescriptionIconsPackerApplication").then((it) => ({ default: it.DescriptionIconsPackerApplication }))
  ),
  preload: () => import("./DescriptionIconsPackerApplication"),
  description: "Build a description sprite from individual icons",
  group: EApplicationGroupId.ICONS,
  icon: <Inventory2Icon />,
  id: EApplicationId.DESCRIPTION_ICONS_PACKER,
  label: "Description icons packer",
  path: "/description-icons-packer",
  status: EApplicationStatus.PLANNED,
};
