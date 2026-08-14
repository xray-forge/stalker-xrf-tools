import { default as MapIcon } from "@mui/icons-material/Map";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";
import { SpawnFileService } from "@/core/spawn/services";

export const SPAWN_EDITOR_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [SpawnFileService] },
  Component: lazy(() => import("./SpawnEditorApplication").then((it) => ({ default: it.SpawnEditorApplication }))),
  preload: () => import("@/applications/spawn-editor/SpawnEditorApplication"),
  description: "Browse and edit a packed spawn file",
  group: EApplicationGroupId.SPAWNS,
  icon: <MapIcon />,
  id: EApplicationId.SPAWN_EDITOR,
  label: "Spawn editor",
  path: "/spawn-editor",
  status: EApplicationStatus.READY,
};
