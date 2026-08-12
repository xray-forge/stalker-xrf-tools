import { default as MapIcon } from "@mui/icons-material/Map";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { SpawnFileService } from "@/lib/spawn-file";

export const SPAWN_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
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
