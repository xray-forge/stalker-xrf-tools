import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect } from "react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath } from "@/lib/xrf-path";

export function ArchivesEditorOpenForm(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);
  const projectService: ProjectService = useInjection(ProjectService);

  const log: Logger = useLogger("archives-editor");

  const isLoading: boolean = archivesService.project.isLoading;

  const [archivesPath, setArchivesPath, onSelectArchivesPath] = usePathState({
    title: "Provide path to packed archives",
    isDirectory: true,
    isDisabled: isLoading,
  });

  const onOpenPathClicked = useCallback(() => {
    if (archivesPath) {
      archivesService.openArchivesProject(archivesPath);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archivesPath, log, archivesService]);

  const onClearArchivesPath = useCallback(() => setArchivesPath(null), [setArchivesPath]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getExistingProjectLinkedGamePath(projectService.xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
    }
  }, [projectService.xrfProjectPath, setArchivesPath]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Provide archives to open"}
      error={archivesService.project.error ? archivesService.project.error.message : undefined}
      backDisabled={isLoading}
      backPath={"/archives_editor"}
      actions={
        <Button variant={"contained"} disabled={isLoading || !archivesPath} onClick={onOpenPathClicked}>
          Open
        </Button>
      }
    >
      <FilePickerInput
        isDisabled={isLoading}
        isInvalid={Boolean(archivesService.project.error)}
        label={"Archives directory"}
        description={"Directory holding the packed game archives"}
        value={archivesPath}
        onSelect={onSelectArchivesPath}
        onClear={onClearArchivesPath}
      />
    </PickerForm>
  );
}
