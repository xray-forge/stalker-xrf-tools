import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const EXPORTS_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/exports").then((it) => ({
      default: it.ExportsApplication,
    }))
  ),
  description: "Browse typescript extern declarations from an XRF project",
  group: EApplicationGroupId.EXPORTS,
  icon: <SwapHorizIcon />,
  id: EApplicationId.EXPORTS,
  label: "Exports",
  path: "/exports",
  status: EApplicationStatus.READY,
};
