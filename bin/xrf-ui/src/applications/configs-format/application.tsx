import { default as FormatAlignLeftIcon } from "@mui/icons-material/FormatAlignLeft";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const CONFIGS_FORMAT_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/configs-format").then((it) => ({
      default: it.ConfigsFormatApplication,
    }))
  ),
  description: "Check or format LTX configuration files",
  group: EApplicationGroupId.CONFIGS,
  icon: <FormatAlignLeftIcon />,
  id: EApplicationId.CONFIGS_FORMAT,
  label: "Configs formatter",
  path: "/configs-format",
  status: EApplicationStatus.READY,
};
