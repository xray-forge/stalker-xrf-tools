import { default as ForumIcon } from "@mui/icons-material/Forum";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";

export const DIALOGS_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("@/applications/dialogs").then((it) => ({
      default: it.DialogsApplication,
    }))
  ),
  description: "Edit NPC dialog graphs",
  group: EApplicationGroupId.DIALOGS,
  icon: <ForumIcon />,
  id: EApplicationId.DIALOGS,
  label: "Dialogs",
  path: "/dialogs",
  status: EApplicationStatus.PLANNED,
};
