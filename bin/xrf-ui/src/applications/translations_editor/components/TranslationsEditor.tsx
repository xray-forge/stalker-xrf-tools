import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { TranslationsEditorMenu } from "@/applications/translations_editor/components/TranslationsEditorMenu";
import { TranslationsEditorWorkspace } from "@/applications/translations_editor/components/TranslationsEditorWorkspace";
import { TranslationsService } from "@/applications/translations_editor/store/translations";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

export function TranslationsEditor(): ReactElement {
  const translationsService: TranslationsService = useInjection(TranslationsService);

  const fileCount: number = Object.keys(translationsService.project.value ?? {}).length;

  return (
    <EditorLayout
      toolbar={
        <EditorToolbar
          title={"Translations editor"}
          subtitle={`${fileCount} files`}
          backPath={"/translations_editor"}
        />
      }
      menu={<TranslationsEditorMenu />}
    >
      <TranslationsEditorWorkspace />
    </EditorLayout>
  );
}
