import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ExportsService } from "@/applications/exports-editor/store/exports";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";

export function ExportsOpenForm(): ReactElement {
  const log: Logger = useLogger("exports-open");

  const exportsService: ExportsService = useInjection(ExportsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = exportsService.declarations.isLoading;

  const project: IPathField = usePathField({
    id: "exports.open.project",
    title: "Provide path to xrf project",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () => projectService.xrfProjectPath,
  });

  const onOpen = useCallback(() => {
    if (project.value) {
      exportsService.openExports(project.value);
    } else {
      log.info("Cannot open exports without a project path");
    }
  }, [exportsService, log, project.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open script exports"}
      error={exportsService.declarations.error ? String(exportsService.declarations.error) : undefined}
      backPath={"/exports-editor"}
      backDisabled={isLoading}
      submitLabel={"Open exports"}
      isSubmitDisabled={!project.isValid}
      onSubmit={onOpen}
    >
      <PathFormRow
        label={"Project"}
        description={"Root of the xrf project whose script exports are read"}
        isDisabled={isLoading}
        field={project}
      />
    </PickerForm>
  );
}
