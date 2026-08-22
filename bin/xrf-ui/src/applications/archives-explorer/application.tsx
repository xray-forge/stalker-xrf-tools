import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { lazy } from "react";

import { ArchivesService } from "@/applications/archives-explorer/services/archives";
import { ArchiveVisualsService } from "@/applications/archives-explorer/services/visuals";
import { AssetService } from "@/core/assets/services";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const ARCHIVES_EXPLORER_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, ArchivesService, ArchiveVisualsService] },
  Component: lazy(() =>
    import("./ArchivesExplorerApplication").then((it) => ({ default: it.ArchivesExplorerApplication }))
  ),
  preload: () => import("./ArchivesExplorerApplication"),
  description: "Browse files stored in game archives",
  group: EApplicationGroupId.ARCHIVES,
  icon: <ArchiveIcon />,
  id: EApplicationId.ARCHIVES_EXPLORER,
  label: "Archives explorer",
  path: "/archives-explorer",
  status: EApplicationStatus.READY,
};
