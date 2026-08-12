import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";

export const VISUAL_PREVIEW_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/visual-preview").then((it) => ({
      default: it.VisualPreviewApplication,
    }))
  ),
  description: "Preview a game visual in 3D",
  group: EApplicationGroupId.VISUALS,
  icon: <ViewInArIcon />,
  id: EApplicationId.VISUAL_PREVIEW,
  label: "Visual preview",
  path: "/visual-preview",
  status: EApplicationStatus.READY,
};
