import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { lazy } from "react";

import { ArchivesService } from "@/applications/archives/services/archives";
import { AssetService } from "@/core/assets/services";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const ARCHIVES_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, ArchivesService] },
  Component: lazy(() =>
    import("@/applications/archives").then((it) => ({
      default: it.ArchivesApplication,
    }))
  ),
  preload: () => import("@/applications/archives"),
  description: "Browse files stored in game archives",
  group: EApplicationGroupId.ARCHIVES,
  icon: <ArchiveIcon />,
  id: EApplicationId.ARCHIVES,
  label: "Archives",
  path: "/archives",
  status: EApplicationStatus.READY,
};
