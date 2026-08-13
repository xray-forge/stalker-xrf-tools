import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";
import { SpawnFileService } from "@/lib/xrf/spawn-file";

export const SPAWN_UNPACK_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
  Component: lazy(() =>
    import("@/applications/spawn-unpack").then((it) => ({
      default: it.SpawnUnpackApplication,
    }))
  ),
  preload: () => import("@/applications/spawn-unpack"),
  description: "Extract a spawn file into editable chunks",
  group: EApplicationGroupId.SPAWNS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.SPAWN_UNPACK,
  label: "Spawn unpack",
  path: "/spawn-unpack",
  status: EApplicationStatus.READY,
};
