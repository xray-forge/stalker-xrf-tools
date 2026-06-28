import { default as FolderIcon } from "@mui/icons-material/Folder";
import { Button, IconButton, InputAdornment, OutlinedInput } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { useCallback, useState } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";

export function ConfigsEditorExplorerPage() {
  const { xrfConfigsPath } = useInjection(ProjectService);

  const [configsPath] = useState<Optional<string>>(xrfConfigsPath);

  const onSelectTargetDirectory = useCallback(() => {}, []);

  const onSelectTargetDirectoryClicked = useCallback(() => {}, []);

  return (
    <PickerForm
      title={"Provide LTX files directory to open"}
      backPath={"/configs_editor"}
      actions={
        <Button variant={"contained"} fullWidth>
          Open
        </Button>
      }
    >
      <OutlinedInput
        size={"small"}
        placeholder={"Configs directory"}
        readOnly={true}
        endAdornment={
          <InputAdornment position={"end"} onClick={onSelectTargetDirectory}>
            <IconButton edge={"end"}>
              <FolderIcon />
            </IconButton>
          </InputAdornment>
        }
        value={configsPath || ""}
        onClick={onSelectTargetDirectoryClicked}
      />
    </PickerForm>
  );
}
