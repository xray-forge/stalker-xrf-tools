import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, FormControl, IconButton, InputAdornment, InputLabel, OutlinedInput } from "@mui/material";
import { open } from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback } from "react";

import { ExportsService } from "@/applications/exports_editor/store/exports";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function ExportsOpenForm(): ReactElement {
  const log: Logger = useLogger("exports-open");

  const { declarations, openExports } = useInjection(ExportsService);
  const { xrfProjectPath, setXrfProjectPath } = useInjection(ProjectService);

  const onSelectProjectPath = useCallback(
    async (event: MouseEvent<HTMLInputElement>) => {
      event.stopPropagation();
      event.preventDefault();

      const newXrfProjectPath: Optional<string> = (await open({
        title: "Provide path to xrf project",
        directory: true,
      })) as Optional<string>;

      if (newXrfProjectPath) {
        log.info("Selected new project path:", newXrfProjectPath);
        setXrfProjectPath(newXrfProjectPath);
      }
    },
    [log, setXrfProjectPath]
  );

  const onSelectProjectPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectProjectPath(event),
    [onSelectProjectPath]
  );

  const onOpenExportsClicked = useCallback(() => {
    if (xrfProjectPath) {
      openExports(xrfProjectPath);
    } else {
      log.info("Cannot open exports when have no project path");
    }
  }, [log, openExports, xrfProjectPath]);

  return (
    <PickerForm
      title={"Provide paths to ltx project"}
      error={declarations.error ? String(declarations.error) : undefined}
      isLoading={declarations.isLoading}
      backPath={"/exports_editor"}
      actions={
        <Button
          fullWidth={true}
          disabled={!xrfProjectPath || declarations.isLoading}
          variant={"contained"}
          onClick={onOpenExportsClicked}
        >
          Open exports
        </Button>
      }
    >
      <FormControl size={"small"} variant={"outlined"}>
        <InputLabel size={"small"}>Project</InputLabel>
        <OutlinedInput
          size={"small"}
          type={"text"}
          endAdornment={
            <InputAdornment position={"end"} onClick={onSelectProjectPath}>
              <IconButton edge={"end"}>
                <FolderIcon />
              </IconButton>
            </InputAdornment>
          }
          label={"Project"}
          value={xrfProjectPath || ""}
          readOnly={true}
          onClick={onSelectProjectPathClicked}
        />
      </FormControl>
    </PickerForm>
  );
}
