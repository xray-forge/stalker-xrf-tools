import { default as ForumIcon } from "@mui/icons-material/Forum";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const DIALOGS_APPLICATION: IApplication = {
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
