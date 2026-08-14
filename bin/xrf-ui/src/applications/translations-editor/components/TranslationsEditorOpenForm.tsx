import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { TranslationsService } from "@/applications/translations-editor/store/translations";
import { EApplicationId } from "@/core/routing/application";
import { getPathIfExists, getProjectTranslationsPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { PickerForm } from "@/core/shell/editor/PickerForm";
import { PathFormRow } from "@/core/ui/form/PathFormRow";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";

export function TranslationsEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("translations-open");

  const translationsService: TranslationsService = useInjection(TranslationsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = translationsService.project.isLoading;

  const translations: IPathField = usePathField({
    application: EApplicationId.TRANSLATIONS_EDITOR,
    id: "directory",
    title: "Select translations directory",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getPathIfExists(getProjectTranslationsPath(projectService.xrfProjectPath)) : null,
  });

  const onOpen = useCallback(() => {
    if (translations.value) {
      translationsService.openProject(translations.value);
    } else {
      log.info("Cannot open translations without a path");
    }
  }, [log, translationsService, translations.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={!translations.isValid}
      title={"Open translations"}
      description={"Reads the localization tables into the editor. Nothing is written until you save."}
      error={translationsService.project.error ? String(translationsService.project.error) : undefined}
      submitLabel={"Open"}
      onSubmit={onOpen}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Translations directory"}
        description={"Directory holding the localization tables"}
        field={translations}
      />
    </PickerForm>
  );
}
