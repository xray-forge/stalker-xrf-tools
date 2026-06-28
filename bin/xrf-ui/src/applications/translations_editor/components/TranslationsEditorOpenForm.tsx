import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect } from "react";

import { TranslationsService } from "@/applications/translations_editor/store/translations";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectTranslationsPath } from "@/lib/xrf_path";

export function TranslationsEditorOpenForm(): ReactElement {
  const log: Logger = useLogger("translations-editor-open");

  const translationsService: TranslationsService = useInjection(TranslationsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [translationsPath, setTranslationsPath, onSelectTranslationsPath] = usePathState({
    title: "Provide path to equipment_editor dds",
    filters: [{ name: "dds", extensions: ["dds"] }],
    isDisabled: translationsService.project.isLoading,
  });

  const onOpenTranslationsClicked = useCallback(() => {
    if (translationsPath) {
      translationsService.openTranslationsProject(translationsPath);
    } else {
      log.info("Cannot open translations when have no provided paths:", {
        translationsPath,
      });
    }
  }, [log, translationsService, translationsPath]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getPathIfExists(getProjectTranslationsPath(projectService.xrfProjectPath)).then((translationsPath) => {
        setTranslationsPath(translationsPath);
      });
    }
  }, []);

  return (
    <PickerForm
      title={"Provide translations details"}
      error={translationsService.project.error ? String(translationsService.project.error) : undefined}
      isLoading={translationsService.project.isLoading}
      backPath={"/translations_editor"}
      actions={
        <Button
          fullWidth
          disabled={translationsService.project.isLoading || !translationsPath}
          variant={"contained"}
          onClick={onOpenTranslationsClicked}
        >
          Open
        </Button>
      }
    >
      <FilePickerInput
        label={"Translations path"}
        value={translationsPath}
        disabled={translationsService.project.isLoading}
        onClick={onSelectTranslationsPath}
      />
    </PickerForm>
  );
}
