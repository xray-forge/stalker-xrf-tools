import { default as ImageIcon } from "@mui/icons-material/Image";
import { lazy } from "react";

import { AssetService } from "@/core/assets/services";
import { EquipmentService } from "@/core/equipment-icons";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const EQUIPMENT_ICONS_EDITOR_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [AssetService, EquipmentService] },
  Component: lazy(() =>
    import("./EquipmentIconsEditorApplication").then((it) => ({ default: it.EquipmentIconsEditorApplication }))
  ),
  preload: () => import("./EquipmentIconsEditorApplication"),
  description: "Inspect and edit equipment inventory icons",
  group: EApplicationGroupId.ICONS,
  icon: <ImageIcon />,
  id: EApplicationId.EQUIPMENT_ICONS_EDITOR,
  label: "Equipment icons editor",
  path: "/equipment-icons-editor",
  status: EApplicationStatus.READY,
};
