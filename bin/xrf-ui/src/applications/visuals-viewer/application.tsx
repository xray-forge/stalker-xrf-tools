import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { lazy } from "react";

import { VisualsService } from "@/applications/visuals-viewer/store/visuals";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const VISUALS_VIEWER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() => import("./VisualsViewerApplication").then((it) => ({ default: it.VisualsViewerApplication }))),
  container: { bindings: [VisualsService] },
  preload: () => import("./VisualsViewerApplication"),
  description: "Preview a game visual in 3D",
  group: EApplicationGroupId.VISUALS,
  icon: <ViewInArIcon />,
  id: EApplicationId.VISUALS_VIEWER,
  label: "Visuals viewer",
  path: "/visuals-viewer",
  status: EApplicationStatus.READY,
};
