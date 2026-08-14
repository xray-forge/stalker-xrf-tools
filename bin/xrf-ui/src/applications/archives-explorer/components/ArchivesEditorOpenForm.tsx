import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchivesService } from "@/applications/archives-explorer/services/archives";
import { EApplicationId } from "@/core/routing/application";
import { getExistingProjectLinkedGamePath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { PickerForm } from "@/core/shell/editor/PickerForm";
import { PathFormRow } from "@/core/ui/form/PathFormRow";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";

export function ArchivesEditorOpenForm(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);
  const projectService: ProjectService = useInjection(ProjectService);

  const log: Logger = useLogger("archives");

  const isLoading: boolean = archivesService.project.isLoading;

  const archives: IPathField = usePathField({
    application: EApplicationId.ARCHIVES_EXPLORER,
    id: "source",
    title: "Select archives directory",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectLinkedGamePath(projectService.xrfProjectPath) : null,
  });

  const onOpen = useCallback(() => {
    if (archives.value) {
      archivesService.openProject(archives.value);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archives.value, log, archivesService]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open game archives"}
      description={"Indexes every archive in the directory for browsing. Nothing is written."}
      error={archivesService.project.error ? archivesService.project.error.message : undefined}
      submitLabel={"Open"}
      isSubmitDisabled={!archives.isValid}
      onSubmit={onOpen}
    >
      <PathFormRow
        label={"Archives directory"}
        description={"Directory holding the packed game archives"}
        isDisabled={isLoading}
        field={archives}
      />
    </PickerForm>
  );
}
