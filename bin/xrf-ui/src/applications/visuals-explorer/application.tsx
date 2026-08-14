import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { lazy } from "react";

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
  preload: () => import("./VisualsExplorerApplication"),
  description: "Browse visuals referenced by a project",
  group: EApplicationGroupId.VISUALS,
  icon: <AccountTreeIcon />,
  id: EApplicationId.VISUALS_EXPLORER,
  label: "Visuals explorer",
  path: "/visuals-explorer",
  status: EApplicationStatus.READY,
};
