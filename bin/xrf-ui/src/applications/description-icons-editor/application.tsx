import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const DESCRIPTION_ICONS_EDITOR_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./DescriptionIconsEditorApplication").then((it) => ({ default: it.DescriptionIconsEditorApplication }))
  ),
  preload: () => import("./DescriptionIconsEditorApplication"),
  description: "Inspect and edit item description icons",
  group: EApplicationGroupId.ICONS,
  icon: <DescriptionIcon />,
  id: EApplicationId.DESCRIPTION_ICONS_EDITOR,
  label: "Description icons editor",
  path: "/description-icons-editor",
  status: EApplicationStatus.PLANNED,
};
