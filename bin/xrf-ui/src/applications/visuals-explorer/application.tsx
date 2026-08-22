import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";
import { lazy } from "react";

import { VisualsBrowseService } from "@/applications/visuals-explorer/store/browse";
import { VisualsService } from "@/applications/visuals-explorer/store/visuals";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const VISUALS_EXPLORER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./VisualsExplorerApplication").then((it) => ({ default: it.VisualsExplorerApplication }))
  ),
  container: { bindings: [VisualsService, VisualsBrowseService] },
  preload: () => import("./VisualsExplorerApplication"),
  description: "Browse and preview game visuals in 3D",
  group: EApplicationGroupId.VISUALS,
  icon: <ViewInArIcon />,
  id: EApplicationId.VISUALS_EXPLORER,
  label: "Visuals explorer",
  path: "/visuals-explorer",
  status: EApplicationStatus.READY,
};
