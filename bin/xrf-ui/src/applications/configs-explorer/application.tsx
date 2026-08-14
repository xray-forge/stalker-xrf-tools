import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const CONFIGS_EXPLORER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./ConfigsExplorerApplication").then((it) => ({ default: it.ConfigsExplorerApplication }))
  ),
  preload: () => import("./ConfigsExplorerApplication"),
  description: "Browse LTX configuration files",
  group: EApplicationGroupId.CONFIGS,
  icon: <FolderOpenIcon />,
  id: EApplicationId.CONFIGS_EXPLORER,
  label: "Configs explorer",
  path: "/configs-explorer",
  status: EApplicationStatus.PLANNED,
};
