import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { TranslationsEditorWorkspace } from "@/applications/translations_editor/components/TranslationsEditorWorkspace";
import { TranslationsService } from "@/applications/translations_editor/store/translations";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditor(): ReactElement {
  const log: Logger = useLogger("translations-editor");

  const translationsService: TranslationsService = useInjection(TranslationsService);

  const navigate: NavigateFunction = useNavigate();

  const fileCount: number = Object.keys(translationsService.project.value ?? {}).length;

  useEditorStatus([`${fileCount} files`]);

  const onClose = useCallback(async () => {
    log.info("Closing translations");

    await translationsService.closeTranslationsProject();

    navigate("/translations_editor", { replace: true });
  }, [log, navigate, translationsService]);

  return (
    <EditorLayout
      toolbar={<EditorToolbar onBack={onClose} />}
    >
      <TranslationsEditorWorkspace />
    </EditorLayout>
  );
}
