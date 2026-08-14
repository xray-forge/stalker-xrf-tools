import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";
import { SpawnFileService } from "@/core/spawn/services";

export const SPAWN_UNPACKER_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
  Component: lazy(() => import("./SpawnUnpackerApplication").then((it) => ({ default: it.SpawnUnpackerApplication }))),
  preload: () => import("./SpawnUnpackerApplication"),
  description: "Extract a spawn file into editable chunks",
  group: EApplicationGroupId.SPAWNS,
  icon: <UnarchiveIcon />,
  id: EApplicationId.SPAWN_UNPACKER,
  label: "Spawn unpacker",
  path: "/spawn-unpacker",
  status: EApplicationStatus.READY,
};
