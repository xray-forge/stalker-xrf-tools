import { default as CloseIcon } from "@mui/icons-material/Close";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { TranslationsService } from "@/applications/translations_editor/store/translations";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditorMenu(): ReactElement {
  const log: Logger = useLogger("translations-editor-menu");
  const navigate: NavigateFunction = useNavigate();

  const translationsService: TranslationsService = useInjection(TranslationsService);

  const onCloseClick = useCallback(async () => {
    log.info("Closing translations");

    await translationsService.closeTranslationsProject();

    navigate("/translations_editor", { replace: true });
  }, [log, navigate, translationsService]);

  const actions: Array<IEditorSideMenuItem> = useMemo(
    () => [
      {
        label: "Close",
        icon: <CloseIcon />,
        isDisabled: translationsService.project.isLoading,
        onClick: onCloseClick,
      },
    ],
    [translationsService.project.isLoading, onCloseClick]
  );

  if (!translationsService.project.value) {
    throw new Error("Unexpected rendering of translations menu.");
  }

  return <EditorSideMenu actions={actions} />;
}
