import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { lazy } from "react";

import { EApplicationGroupId, EApplicationId, EApplicationStatus, IApplication } from "@/core/router/application";

export const TRANSLATIONS_APPLICATION: IApplication = {
  Component: lazy(() =>
    import("@/applications/translations").then((it) => ({
      default: it.TranslationsApplication,
    }))
  ),
  description: "Browse and edit localization tables",
  group: EApplicationGroupId.TRANSLATIONS,
  icon: <TranslateIcon />,
  id: EApplicationId.TRANSLATIONS,
  label: "Translations",
  path: "/translations",
  status: EApplicationStatus.READY,
};
