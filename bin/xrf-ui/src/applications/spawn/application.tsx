import { default as MapIcon } from "@mui/icons-material/Map";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const SPAWN_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/spawn").then((it) => ({
      default: it.SpawnApplication,
    }))
  ),
  description: "Browse and edit a packed spawn file",
  group: EApplicationGroupId.SPAWNS,
  icon: <MapIcon />,
  id: EApplicationId.SPAWN,
  label: "Spawn editor",
  path: "/spawn",
  status: EApplicationStatus.READY,
};
