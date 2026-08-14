import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";
import { SpawnFileService } from "@/core/spawn/services";

export const SPAWN_PACKER_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
  Component: lazy(() => import("./SpawnPackerApplication").then((it) => ({ default: it.SpawnPackerApplication }))),
  preload: () => import("./SpawnPackerApplication"),
  description: "Build a spawn file from unpacked chunks",
  group: EApplicationGroupId.SPAWNS,
  icon: <Inventory2Icon />,
  id: EApplicationId.SPAWN_PACKER,
  label: "Spawn packer",
  path: "/spawn-packer",
  status: EApplicationStatus.READY,
};
