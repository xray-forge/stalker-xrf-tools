import { default as FactCheckIcon } from "@mui/icons-material/FactCheck";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const CONFIGS_VERIFY_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/configs-verify").then((it) => ({
      default: it.ConfigsVerifyApplication,
    }))
  ),
  description: "Validate LTX configuration files",
  group: EApplicationGroupId.CONFIGS,
  icon: <FactCheckIcon />,
  id: EApplicationId.CONFIGS_VERIFY,
  label: "Configs verifier",
  path: "/configs-verify",
  status: EApplicationStatus.READY,
};
