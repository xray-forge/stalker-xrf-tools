import { default as ForumIcon } from "@mui/icons-material/Forum";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const DIALOGS_EDITOR_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() => import("./DialogsEditorApplication").then((it) => ({ default: it.DialogsEditorApplication }))),
  preload: () => import("./DialogsEditorApplication"),
  description: "Edit NPC dialog graphs",
  group: EApplicationGroupId.DIALOGS,
  icon: <ForumIcon />,
  id: EApplicationId.DIALOGS_EDITOR,
  label: "Dialogs editor",
  path: "/dialogs-editor",
  status: EApplicationStatus.PLANNED,
};
