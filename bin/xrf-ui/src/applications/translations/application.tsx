import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { lazy } from "react";

import { TranslationsService } from "@/applications/translations/store/translations";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";

export const TRANSLATIONS_APPLICATION: IApplicationDescriptor = {
  container: { bindings: [TranslationsService] },
  Component: lazy(() =>
    import("@/applications/translations").then((it) => ({
      default: it.TranslationsApplication,
    }))
  ),
  preload: () => import("@/applications/translations"),
  description: "Browse and edit localization tables",
  group: EApplicationGroupId.TRANSLATIONS,
  icon: <TranslateIcon />,
  id: EApplicationId.TRANSLATIONS,
  label: "Translations",
  path: "/translations",
  status: EApplicationStatus.READY,
};
