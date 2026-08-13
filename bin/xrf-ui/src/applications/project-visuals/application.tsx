import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const PROJECT_VISUALS_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/project-visuals").then((it) => ({
      default: it.ProjectVisualsApplication,
    }))
  ),
  preload: () => import("@/applications/project-visuals"),
  description: "Browse visuals referenced by a project",
  group: EApplicationGroupId.VISUALS,
  icon: <AccountTreeIcon />,
  id: EApplicationId.PROJECT_VISUALS,
  label: "Project visuals",
  path: "/project-visuals",
  status: EApplicationStatus.READY,
};
