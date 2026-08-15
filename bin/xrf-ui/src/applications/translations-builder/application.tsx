import { default as BuildIcon } from "@mui/icons-material/Build";
import { lazy } from "react";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";

export const TRANSLATIONS_BUILDER_APPLICATION: IApplicationDescriptor = {
  Component: lazy(() =>
    import("./TranslationsBuilderApplication").then((it) => ({ default: it.TranslationsBuilderApplication }))
  ),
  preload: () => import("./TranslationsBuilderApplication"),
  description: "Build per-language string tables from translation sources",
  group: EApplicationGroupId.TRANSLATIONS,
  icon: <BuildIcon />,
  id: EApplicationId.TRANSLATIONS_BUILDER,
  label: "Translations builder",
  path: "/translations-builder",
  status: EApplicationStatus.PLANNED,
};
