import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { lazy } from "react";

import { ExportsService } from "@/applications/exports/store/exports";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EXPORTS_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [ExportsService] },
  Component: lazy(() =>
    import("@/applications/exports").then((it) => ({
      default: it.ExportsApplication,
    }))
  ),
  preload: () => import("@/applications/exports"),
  description: "Browse typescript extern declarations from an XRF project",
  group: EApplicationGroupId.EXPORTS,
  icon: <SwapHorizIcon />,
  id: EApplicationId.EXPORTS,
  label: "Exports",
  path: "/exports",
  status: EApplicationStatus.READY,
};
