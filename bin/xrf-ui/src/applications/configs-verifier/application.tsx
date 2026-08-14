import { default as FactCheckIcon } from "@mui/icons-material/FactCheck";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const CONFIGS_VERIFIER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./ConfigsVerifierApplication").then((it) => ({ default: it.ConfigsVerifierApplication }))
  ),
  preload: () => import("./ConfigsVerifierApplication"),
  description: "Validate LTX configuration files",
  group: EApplicationGroupId.CONFIGS,
  icon: <FactCheckIcon />,
  id: EApplicationId.CONFIGS_VERIFIER,
  label: "Configs verifier",
  path: "/configs-verifier",
  status: EApplicationStatus.READY,
};
