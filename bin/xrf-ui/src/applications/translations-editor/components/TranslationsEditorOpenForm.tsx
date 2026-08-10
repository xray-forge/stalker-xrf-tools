import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { TranslationsService } from "@/applications/translations-editor/store/translations";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectTranslationsPath } from "@/lib/xrf-path";

export function TranslationsEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("translations-editor-open");

  const translationsService: TranslationsService = useInjection(TranslationsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = translationsService.project.isLoading;

  const translations: IPathField = usePathField({
    id: "translations.open.directory",
    title: "Provide path to translations",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath
        ? getPathIfExists(getProjectTranslationsPath(projectService.xrfProjectPath))
        : null,
  });

  const onOpen = useCallback(() => {
    if (translations.value) {
      translationsService.openTranslationsProject(translations.value);
    } else {
      log.info("Cannot open translations without a path");
    }
  }, [log, translationsService, translations.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open translations"}
      error={translationsService.project.error ? String(translationsService.project.error) : undefined}
      backPath={"/translations_editor"}
      backDisabled={isLoading}
      submitLabel={"Open"}
      isSubmitDisabled={!translations.isValid}
      onSubmit={onOpen}
    >
      <PathFormRow
        label={"Translations directory"}
        description={"Directory holding the localization tables"}
        isDisabled={isLoading}
        field={translations}
      />
    </PickerForm>
  );
}
