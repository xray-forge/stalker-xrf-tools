import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { SpawnFileService } from "@/lib/spawn-file";

export const SPAWN_PACK_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
  Component: lazy(() =>
    import("@/applications/spawn-pack").then((it) => ({
      default: it.SpawnPackApplication,
    }))
  ),
  description: "Build a spawn file from unpacked chunks",
  group: EApplicationGroupId.SPAWNS,
  icon: <Inventory2Icon />,
  id: EApplicationId.SPAWN_PACK,
  label: "Spawn pack",
  path: "/spawn-pack",
  status: EApplicationStatus.READY,
};
