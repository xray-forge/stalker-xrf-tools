import { Button } from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ExportsService } from "@/applications/exports_editor/store/exports";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { Logger, useLogger } from "@/lib/logging";

export function ExportsOpenForm(): ReactElement {
  const log: Logger = useLogger("exports-open");

  const exportsService: ExportsService = useInjection(ExportsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const isLoading: boolean = exportsService.declarations.isLoading;

  // This screen writes straight to the shared project path rather than holding a local one, so it
  // cannot use `usePathState`: the value it shows belongs to `ProjectService`.
  const onSelectProjectPath = useCallback(async () => {
    if (isLoading) {
      return;
    }

    const newXrfProjectPath: Optional<string> = await open({
      title: "Provide path to xrf project",
      directory: true,
    });

    if (newXrfProjectPath) {
      log.info("Selected new project path:", newXrfProjectPath);
      projectService.setXrfProjectPath(newXrfProjectPath);
    }
  }, [isLoading, log, projectService]);

  const onOpenExportsClicked = useCallback(() => {
    if (projectService.xrfProjectPath) {
      exportsService.openExports(projectService.xrfProjectPath);
    } else {
      log.info("Cannot open exports when have no project path");
    }
  }, [exportsService, log, projectService.xrfProjectPath]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Provide paths to ltx project"}
      error={exportsService.declarations.error ? String(exportsService.declarations.error) : undefined}
      backPath={"/exports_editor"}
      backDisabled={isLoading}
      actions={
        <Button
          disabled={!projectService.xrfProjectPath || isLoading}
          variant={"contained"}
          onClick={onOpenExportsClicked}
        >
          Open exports
        </Button>
      }
    >
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(exportsService.declarations.error)}
        label={"Project"}
        description={"Root of the xrf project whose script exports are read"}
        value={projectService.xrfProjectPath}
        onSelect={onSelectProjectPath}
      />
    </PickerForm>
  );
}
