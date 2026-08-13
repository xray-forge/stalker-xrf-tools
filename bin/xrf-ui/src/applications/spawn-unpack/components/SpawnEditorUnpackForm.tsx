import { Alert } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { PathFormRow } from "@/core/components/form/PathFormRow";
import { IPathField, usePathField } from "@/core/components/form/use-path-field";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ENotificationSeverity, TNotify, useNotify } from "@/core/notifications";
import { EApplicationId } from "@/core/routing/application";
import { getExistingProjectBuiltAllSpawnPath, getProjectAllSpawnUnpackPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { Logger, useLogger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";
import { commands as spawnsEditorCommands } from "@/lib/xrf/bindings/xrf-app-spawns-editor";

/**
 * Expand a packed spawn file into chunks on disk.
 */
export function SpawnEditorUnpackForm(): ReactElement {
  const log: Logger = useLogger("spawn-unpack");
  const notify: TNotify = useNotify();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [unpackedTo, setUnpackedTo] = useState<Nullable<string>>(null);

  const source: IPathField = usePathField({
    id: "spawn.unpack.source",
    title: "Select spawn file",
    filters: [{ name: "spawn", extensions: ["spawn"] }],
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectBuiltAllSpawnPath(projectService.xrfProjectPath) : null,
  });

  const destination: IPathField = usePathField({
    id: "spawn.unpack.destination",
    title: "Select output folder",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getProjectAllSpawnUnpackPath(projectService.xrfProjectPath) : null,
  });

  const onUnpack = useCallback(async () => {
    if (!source.value || !destination.value) {
      return log.error("Cannot unpack spawn file, expected correct paths");
    }

    log.info("Unpacking spawn file:", source.value, destination.value);

    setIsLoading(true);
    setError(null);
    setUnpackedTo(null);

    try {
      await spawnsEditorCommands.unpackSpawnFile(source.value, destination.value);

      setUnpackedTo(destination.value);

      notify({
        details: `${source.value}\n${destination.value}`,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.SPAWN_UNPACK,
        title: "Unpacked spawn file",
      });
    } catch (caught: unknown) {
      log.error("Failed to unpack spawn file:", caught);
      setError(String(caught));

      notify({
        details: `${source.value}\n${String(caught)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.SPAWN_UNPACK,
        title: "Could not unpack spawn file",
      });
    } finally {
      setIsLoading(false);
    }
  }, [destination.value, log, notify, source.value]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={!source.isValid || !destination.isValid}
      title={"Unpack spawn file"}
      error={error ?? undefined}
      submitLabel={"Unpack"}
      status={
        unpackedTo ? (
          <Alert severity={"success"} variant={"outlined"}>
            Successfully unpacked spawn to {unpackedTo}
          </Alert>
        ) : null
      }
      onSubmit={onUnpack}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Source"}
        description={"The packed *.spawn file to read"}
        field={source}
      />

      <PathFormRow
        isDisabled={isLoading}
        label={"Destination"}
        description={"Directory the unpacked chunks are written to"}
        field={destination}
      />
    </PickerForm>
  );
}
