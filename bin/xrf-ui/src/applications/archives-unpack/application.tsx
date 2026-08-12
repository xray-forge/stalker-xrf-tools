import { default as UnarchiveIcon } from "@mui/icons-material/Unarchive";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";

export const ARCHIVES_UNPACK_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/archives-unpack").then((it) => ({
      default: it.ArchivesUnpackApplication,
    }))
  ),
  description: "Extract game archives into a directory",
  group: EApplicationGroupId.ARCHIVES,
  icon: <UnarchiveIcon />,
  id: EApplicationId.ARCHIVES_UNPACK,
  label: "Archives unpack",
  path: "/archives-unpack",
  status: EApplicationStatus.READY,
};
