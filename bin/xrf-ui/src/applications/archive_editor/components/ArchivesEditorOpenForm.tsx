import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, IconButton, InputAdornment, OutlinedInput } from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getExistingProjectLinkedGamePath } from "@/lib/xrf_path";

export function ArchivesEditorOpenForm(): ReactElement {
  const { project, openArchivesProject } = useInjection(ArchivesService);
  const { xrfProjectPath } = useInjection(ProjectService);

  const log: Logger = useLogger("archives-editor");
  const [archivesPath, setArchivesPath] = useState<Optional<string>>(null);

  const onSelectConfigsPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      if (project.isLoading) {
        return;
      }

      event.stopPropagation();
      event.preventDefault();

      const newXrfConfigsPath: Optional<string> = (await open({
        title: "Provide path to packed archives",
        directory: true,
      })) as Optional<string>;

      if (newXrfConfigsPath) {
        log.info("Selected new archives path:", newXrfConfigsPath);

        setArchivesPath(newXrfConfigsPath);
      }
    },
    [log, project.isLoading]
  );

  const onSelectArchivesPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectConfigsPath(event),
    [onSelectConfigsPath]
  );

  const onOpenPathClicked = useCallback(async () => {
    if (archivesPath) {
      openArchivesProject(archivesPath);
    } else {
      log.info("Cannot parse archives project without path");
    }
  }, [archivesPath, log, openArchivesProject]);

  useEffect(() => {
    if (xrfProjectPath) {
      getExistingProjectLinkedGamePath(xrfProjectPath).then((gamePath) => setArchivesPath(gamePath));
    }
  }, [xrfProjectPath]);

  return (
    <PickerForm
      title={"Provide archives to open"}
      error={project.error ? project.error.message : undefined}
      isLoading={project.isLoading}
      backDisabled={project.isLoading}
      backPath={"/archives_editor"}
      actions={
        <Button
          variant={"contained"}
          fullWidth={true}
          disabled={project.isLoading || !archivesPath}
          onClick={onOpenPathClicked}
        >
          Open
        </Button>
      }
    >
      <OutlinedInput
        size={"small"}
        disabled={project.isLoading}
        value={archivesPath || ""}
        placeholder={"Source"}
        readOnly={true}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectConfigsPath}>
            <IconButton disabled={project.isLoading} edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        onClick={onSelectArchivesPathClicked}
      />
    </PickerForm>
  );
}
