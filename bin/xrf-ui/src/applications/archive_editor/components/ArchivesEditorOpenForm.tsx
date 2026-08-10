import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath } from "@/lib/xrf-path";

export function ArchivesEditorOpenForm(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);
  const projectService: ProjectService = useInjection(ProjectService);

  const log: Logger = useLogger("archives-editor");

  const isLoading: boolean = archivesService.project.isLoading;

  const archives: IPathField = usePathField({
    id: "archives.open.source",
    title: "Provide path to packed archives",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectLinkedGamePath(projectService.xrfProjectPath) : null,
  });

  const onOpen = useCallback(() => {
    if (archives.value) {
      archivesService.openArchivesProject(archives.value);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archives.value, log, archivesService]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open game archives"}
      error={archivesService.project.error ? archivesService.project.error.message : undefined}
      backDisabled={isLoading}
      backPath={"/archives_editor"}
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
