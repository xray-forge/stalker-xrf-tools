import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const SPAWN_UNPACK_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/spawn-unpack").then((it) => ({
      default: it.SpawnUnpackApplication,
    }))
  ),
  description: "Extract a spawn file into editable chunks",
  group: EApplicationGroupId.SPAWNS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.SPAWN_UNPACK,
  label: "Spawn unpack",
  path: "/spawn-unpack",
  status: EApplicationStatus.READY,
};
