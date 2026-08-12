import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const ARCHIVES_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/archives").then((it) => ({
      default: it.ArchivesApplication,
    }))
  ),
  description: "Browse files stored in game archives",
  group: EApplicationGroupId.ARCHIVES,
  icon: <ArchiveIcon />,
  id: EApplicationId.ARCHIVES,
  label: "Archives",
  path: "/archives",
  status: EApplicationStatus.READY,
};
