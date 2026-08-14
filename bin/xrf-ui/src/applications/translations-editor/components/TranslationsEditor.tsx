import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { TranslationsEditorWorkspace } from "@/applications/translations-editor/components/TranslationsEditorWorkspace";
import { TranslationsService } from "@/applications/translations-editor/store/translations";
import { EditorLayout } from "@/core/shell/editor/EditorLayout";
import { EditorToolbar } from "@/core/shell/editor/EditorToolbar";
import { useEditorStatus } from "@/core/shell/EditorStatusContext";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditor(): ReactElement {
  const log: Logger = useLogger("translations");

  const translationsService: TranslationsService = useInjection(TranslationsService);

  const fileCount: number = Object.keys(translationsService.project.value ?? {}).length;

  useEditorStatus([`${fileCount} files`]);

  // Closing does not navigate: the application shows its own picker again once nothing is open.
  const onClose = useCallback(async () => {
    log.info("Closing translations");

    await translationsService.closeProject();
  }, [log, translationsService]);

  return (
    <EditorLayout toolbar={<EditorToolbar onBack={onClose} />}>
      <TranslationsEditorWorkspace />
    </EditorLayout>
  );
}
