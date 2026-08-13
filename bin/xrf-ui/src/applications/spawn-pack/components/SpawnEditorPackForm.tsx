import { Alert } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { EApplicationId } from "@/core/router/application";
import { ProjectService } from "@/core/store/project";
import { Nullable } from "@/core/types/general";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";
import { ESpawnsEditorCommand } from "@/lib/xrf/ipc";
import { getExistingProjectUnpackedAllSpawnPath, getProjectAllSpawnRepackPath } from "@/lib/xrf-path";

/**
 * Build a packed spawn file from chunks on disk.
 */
export function SpawnEditorPackForm(): ReactElement {
  const log: Logger = useLogger("spawn-pack");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [packedTo, setPackedTo] = useState<Nullable<string>>(null);

  const source: IPathField = usePathField({
    id: "spawn.pack.source",
    title: "Select unpacked spawn folder",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectUnpackedAllSpawnPath(projectService.xrfProjectPath) : null,
  });

  const destination: IPathField = usePathField({
    id: "spawn.pack.destination",
    title: "Select spawn file output",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isSave: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getProjectAllSpawnRepackPath(projectService.xrfProjectPath) : null,
  });

  const onPack = useCallback(async () => {
    if (!source.value || !destination.value) {
      return log.error("Cannot pack spawn file, expected correct paths");
    }

    log.info("Packing spawn file:", source.value, destination.value);

    setIsLoading(true);
    setError(null);
    setPackedTo(null);

    try {
      await invoke(ESpawnsEditorCommand.PACK_SPAWN_FILE, { from: source.value, destination: destination.value });

      setPackedTo(destination.value);

      notify({
        details: `${source.value}\n${destination.value}`,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.SPAWN_PACK,
        title: "Packed spawn file",
      });
    } catch (caught: unknown) {
      log.error("Failed to pack spawn file:", caught);
      setError(String(caught));

      notify({
        details: `${source.value}\n${String(caught)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.SPAWN_PACK,
        title: "Could not pack spawn file",
      });
    } finally {
      setIsLoading(false);
    }
  }, [destination.value, log, notify, source.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={!source.isValid || !destination.isValid}
      title={"Pack spawn file"}
      error={error ?? undefined}
      submitLabel={"Pack"}
      status={
        packedTo ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully packed spawn to {packedTo}
          </Alert>
        ) : null
      }
      onSubmit={onPack}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Source"}
        description={"Directory holding the unpacked spawn chunks"}
        field={source}
      />

      <PathFormRow
        isDisabled={isLoading}
        label={"Output spawn"}
        description={"Where the packed *.spawn file is written"}
        field={destination}
      />
    </PickerForm>
  );
}
