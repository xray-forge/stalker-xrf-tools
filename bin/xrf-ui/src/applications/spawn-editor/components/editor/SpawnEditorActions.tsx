import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { IconButton, Tooltip } from "@mui/material";
import * as dialog from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { Nullable } from "@/core/types/general";

/**
 * Commands that act on the open spawn file.
 */
export function SpawnEditorActions(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const isLoading: boolean = spawnFileService.spawnFile.isLoading;

  const onSave = useCallback(async () => {
    const path: Nullable<string> = await dialog.save({
      title: "Save spawn file",
      filters: [{ name: "spawn", extensions: ["spawn"] }],
    });

    if (path) {
      await spawnFileService.saveSpawnFile(path);
    }
  }, [spawnFileService]);

  const onExport = useCallback(async () => {
    const path: Nullable<string> = (await dialog.open({
      title: "Export spawn file",
      directory: true,
    })) as Nullable<string>;

    if (path) {
      await spawnFileService.exportSpawnFile(path);
    }
  }, [spawnFileService]);

  return (
    <>
      <Tooltip describeChild title={"Write the open spawn file to a chosen path"}>
        <span>
          <IconButton color={"inherit"} size={"small"} disabled={isLoading} onClick={onSave}>
            <SaveIcon fontSize={"small"} />
          </IconButton>
        </span>
      </Tooltip>

      <Tooltip describeChild title={"Export the spawn file into a directory"}>
        <span>
          <IconButton color={"inherit"} size={"small"} disabled={isLoading} onClick={onExport}>
            <ImportExportIcon fontSize={"small"} />
          </IconButton>
        </span>
      </Tooltip>
    </>
  );
}
