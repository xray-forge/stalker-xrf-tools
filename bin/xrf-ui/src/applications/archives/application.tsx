import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { lazy } from "react";

import { ArchivesService } from "@/applications/archives/store/archives";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { AssetService } from "@/lib/assets";

export const ARCHIVES_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, ArchivesService] },
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
