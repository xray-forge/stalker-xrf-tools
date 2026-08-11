import { default as Inventory2Icon } from "@mui/icons-material/Inventory2";
import { Alert, Box, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo, useState } from "react";

import { EquipmentService } from "@/applications/icons-editor/store/equipment";
import { ConfirmDialog } from "@/core/components/dialog/ConfirmDialog";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";
import { Nullable } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";

export function EquipmentSpriteEditorMenu(): ReactElement {
  const log: Logger = useLogger("equipment-editor-menu");

  const equipmentService: EquipmentService = useInjection(EquipmentService);

  const [isConfirmOpen, setConfirmOpen] = useState<boolean>(false);

  const error: Nullable<Error> = equipmentService.spriteImage.error;
  const isLoading: boolean = equipmentService.spriteImage.isLoading;
  const repackSourcePath: Nullable<string> = equipmentService.repackSourcePath;
  const spritePath: Nullable<string> = equipmentService.spriteImage.value?.path ?? null;

  const onRepack = useCallback(async () => {
    setConfirmOpen(false);

    try {
      await equipmentService.repackAndOpenProject();
    } catch (error) {
      log.error("Failed to repack and reopen DDS:", error);
    }
  }, [log, equipmentService]);

  const actions: Array<IEditorSideMenuItem> = useMemo(
    () => [
      {
        label: "Repack sprite",
        icon: <Inventory2Icon />,
        description: repackSourcePath ? "Rebuild from unpacked icons" : "No unpacked icons beside the sprite",
        isDisabled: isLoading || !repackSourcePath,
        onClick: () => setConfirmOpen(true),
      },
    ],
    [isLoading, repackSourcePath]
  );

  const onDeclineConfirmation = useCallback(() => {
    setConfirmOpen(false);
  }, []);

  return (
    <>
      <EditorSideMenu
        actions={actions}
        footer={
          error ? (
            <Box sx={{ padding: 1 }}>
              <Alert severity={"error"} variant={"outlined"} onClose={equipmentService.clearSpriteError}>
                <Typography variant={"caption"} sx={{ wordBreak: "break-word" }}>
                  {String(error)}
                </Typography>
              </Alert>
            </Box>
          ) : null
        }
      />

      <ConfirmDialog
        isOpen={isConfirmOpen}
        isDestructive={true}
        title={"Repack sprite?"}
        confirmLabel={"Repack"}
        description={
          <>
            The sprite is rebuilt from the icons in
            <Typography component={"div"} variant={"caption"} className={"monospace"} sx={{ paddingY: 0.5 }}>
              {repackSourcePath}
            </Typography>
            overwriting
            <Typography component={"div"} variant={"caption"} className={"monospace"} sx={{ paddingY: 0.5 }}>
              {spritePath}
            </Typography>
            This cannot be undone.
          </>
        }
        onConfirm={onRepack}
        onClose={onDeclineConfirmation}
      />
    </>
  );
}
