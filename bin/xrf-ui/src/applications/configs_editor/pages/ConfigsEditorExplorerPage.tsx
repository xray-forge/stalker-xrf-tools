import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useEffect } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput } from "@/lib/file-picker/FilePickerInput";
import { usePathState } from "@/lib/file-picker/use-path-state";

export function ConfigsEditorExplorerPage(): ReactElement {
  const projectService: ProjectService = useInjection(ProjectService);

  const [configsPath, setConfigsPath, onSelectConfigsPath] = usePathState({
    isDirectory: true,
    title: "Provide path to xrf configs",
  });

  useEffect(() => {
    setConfigsPath(projectService.xrfConfigsPath);
  }, [projectService.xrfConfigsPath, setConfigsPath]);

  return (
    <PickerForm
      title={"Provide LTX files directory to open"}
      backPath={"/configs_editor"}
      actions={
        // The explorer itself is not implemented yet; picking a directory is all this screen does.
        <Button variant={"contained"} disabled>
          Open
        </Button>
      }
    >
      <FilePickerInput
        label={"Configs directory"}
        description={"Directory of LTX files to browse"}
        value={configsPath}
        onSelect={onSelectConfigsPath}
      />
    </PickerForm>
  );
}
