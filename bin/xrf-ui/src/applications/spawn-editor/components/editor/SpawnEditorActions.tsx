import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { IconButton, Tooltip } from "@mui/material";
import * as dialog from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { ConfirmDialog } from "@/core/components/dialog/ConfirmDialog";
import { Nullable } from "@/core/types/general";

/**
 * Commands that act on the open spawn file.
 */
export function SpawnEditorActions(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const [exportPath, setExportPath] = useState<Nullable<string>>(null);

  const isBusy: boolean = spawnFileService.isBusy;

  const onSave = useCallback(async () => {
    // The save dialog asks about overwriting an existing file itself, so there is no second prompt here.
    const path: Nullable<string> = await dialog.save({
      title: "Save spawn file",
      filters: [{ name: "spawn", extensions: ["spawn"] }],
    });

    if (path) {
      await spawnFileService.saveSpawnFile(path);
    }
  }, [spawnFileService]);

  const onPickExportPath = useCallback(async () => {
    const path: Nullable<string> = (await dialog.open({
      title: "Export spawn file",
      directory: true,
    })) as Nullable<string>;

    if (path) {
      setExportPath(path);
    }
  }, []);

  const onConfirmExport = useCallback(() => {
    const path: Nullable<string> = exportPath;

    setExportPath(null);

    if (path) {
      void spawnFileService.exportSpawnFile(path);
    }
  }, [exportPath, spawnFileService]);

  return (
    <>
      <Tooltip describeChild title={"Write the open spawn file to a chosen path"}>
        <span>
          <IconButton
            aria-label={"Save spawn file"}
            color={"inherit"}
            disabled={isBusy}
            size={"small"}
            onClick={onSave}
          >
            <SaveIcon fontSize={"small"} />
          </IconButton>
        </span>
      </Tooltip>

      <Tooltip describeChild title={"Export the spawn file into a directory"}>
        <span>
          <IconButton
            aria-label={"Export spawn file"}
            color={"inherit"}
            disabled={isBusy}
            size={"small"}
            onClick={onPickExportPath}
          >
            <ImportExportIcon fontSize={"small"} />
          </IconButton>
        </span>
      </Tooltip>

      <ConfirmDialog
        isDestructive
        confirmLabel={"Export"}
        description={`Writes one file per chunk into ${exportPath}, replacing any unpacked spawn already there.`}
        isOpen={exportPath !== null}
        title={"Export spawn file?"}
        onClose={() => setExportPath(null)}
        onConfirm={onConfirmExport}
      />
    </>
  );
}
