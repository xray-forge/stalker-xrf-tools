import { default as FactCheckIcon } from "@mui/icons-material/FactCheck";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const CONFIGS_VERIFY_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/configs-verify").then((it) => ({
      default: it.ConfigsVerifyApplication,
    }))
  ),
  preload: () => import("@/applications/configs-verify"),
  description: "Validate LTX configuration files",
  group: EApplicationGroupId.CONFIGS,
  icon: <FactCheckIcon />,
  id: EApplicationId.CONFIGS_VERIFY,
  label: "Configs verifier",
  path: "/configs-verify",
  status: EApplicationStatus.READY,
};
