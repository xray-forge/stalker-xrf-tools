import { Alert, Switch, TextField } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ChangeEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { ArchivesPackResult } from "@/applications/archives-packer/components/ArchivesPackResult";
import { commands as archivesCommands } from "@/core/bindings/xrf-app-archives";
import { ArchivePackConfig, ArchivePackResult } from "@/core/bindings/xrf-archive";
import { ENotificationSeverity, TEmitNotification, useEmitNotification } from "@/core/notifications/lib";
import { EApplicationId } from "@/core/routing/application";
import { getProjectArchivesPackPath, getProjectGamedataPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { PickerForm } from "@/core/shell/editor/PickerForm";
import { FormRow } from "@/core/ui/form/FormRow";
import { PathFormRow } from "@/core/ui/form/PathFormRow";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";
import { BYTES_PER_MEGABYTE } from "@/lib/memory/size";
import { Nullable } from "@/lib/types/general";

// Volumes are named after this unless the form says otherwise, matching the command line default.
const DEFAULT_ARCHIVE_NAME: string = "gamedata";

// Todo: can we get it from engine / cmd in a sync way?
const VOLUME_SIZE_MAX_MEGABYTES: number = 1_900;

export function ArchivesPackerApplication(): ReactElement {
  const log: Logger = useLogger("archives-packer");
  const notify: TEmitNotification = useEmitNotification();

  const projectService: ProjectService = useInjection(ProjectService);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Nullable<string>>(null);
  const [result, setResult] = useState<Nullable<ArchivePackResult>>(null);
  const [name, setName] = useState<string>(DEFAULT_ARCHIVE_NAME);
  const [isStore, setIsStore] = useState<boolean>(false);
  const [maxSize, setMaxSize] = useState<string>("");

  const source: IPathField = usePathField({
    application: EApplicationId.ARCHIVES_PACKER,
    id: "source",
    title: "Select directory to pack",
    isDirectory: true,
    isDisabled: isLoading,
    seed: async () => (projectService.xrfProjectPath ? getProjectGamedataPath(projectService.xrfProjectPath) : null),
  });

  const destination: IPathField = usePathField({
    application: EApplicationId.ARCHIVES_PACKER,
    id: "destination",
    title: "Select output directory",
    isDirectory: true,
    isSave: true,
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getProjectArchivesPackPath(projectService.xrfProjectPath) : null,
  });

  const config: IPathField = usePathField({
    application: EApplicationId.ARCHIVES_PACKER,
    id: "config",
    title: "Select packing configuration",
    filters: [{ name: "Packing configuration", extensions: ["ltx"] }],
    isDisabled: isLoading,
    isRequired: false,
  });

  const maxSizeValue: number = Number(maxSize);
  const maxSizeError: Nullable<string> =
    maxSize.trim() && (!Number.isInteger(maxSizeValue) || maxSizeValue < 1 || maxSizeValue > VOLUME_SIZE_MAX_MEGABYTES)
      ? `Enter a whole number between 1 and ${VOLUME_SIZE_MAX_MEGABYTES}`
      : null;

  const sourcePath: Nullable<string> = source.value;
  const destinationPath: Nullable<string> = destination.value;
  const configPath: Nullable<string> = config.value;

  const onPackClicked = useCallback(async () => {
    if (!sourcePath || !destinationPath) {
      return;
    }

    try {
      setIsLoading(true);
      setResult(null);
      setError(null);

      log.info("Packing:", sourcePath);

      // Everything the run needs, in the shape the packer and the command line both take. Selection
      // rules stay empty unless a configuration supplies them, and empty means the whole directory.
      let config: ArchivePackConfig = {
        source: sourcePath,
        destination: destinationPath,
        name: name.trim() || DEFAULT_ARCHIVE_NAME,
        includeFiles: [],
        includeFolders: [],
        excludeFolders: [],
        excludeExtensions: [],
        isWithSkipList: true,
        header: null,
        mode: isStore ? "Store" : "Compress",
        maxVolumeSize: (maxSize.trim() ? Number(maxSize) : VOLUME_SIZE_MAX_MEGABYTES) * BYTES_PER_MEGABYTE,
        volumeExtension: "Db",
      };

      if (configPath) {
        config = await archivesCommands.importPackConfig(configPath, config);
      }

      const packed: ArchivePackResult = await archivesCommands.packDirectory(config);

      log.info("Packed:", sourcePath);

      setResult(packed);

      notify({
        details: `${sourcePath}\n${destinationPath}`,
        severity: ENotificationSeverity.SUCCESS,
        source: EApplicationId.ARCHIVES_PACKER,
        title: "Packed archives",
      });
    } catch (error: unknown) {
      log.error("Pack error:", error);
      setError(String(error));

      notify({
        details: `${sourcePath}\n${String(error)}`,
        severity: ENotificationSeverity.ERROR,
        source: EApplicationId.ARCHIVES_PACKER,
        title: "Could not pack archives",
      });
    } finally {
      setIsLoading(false);
    }
  }, [sourcePath, destinationPath, configPath, name, isStore, maxSize, log, notify]);

  // Any change to what would be packed invalidates whatever the previous run reported.
  useEffect(() => {
    setError(null);
    setResult(null);
  }, [sourcePath, destinationPath, configPath, name, isStore, maxSize]);

  return (
    <PickerForm
      isLoading={isLoading}
      isSubmitDisabled={!source.isValid || !destination.isValid || !config.isValid || Boolean(maxSizeError)}
      title={"Pack game archives"}
      description={
        "Writes the source directory into database volumes. Existing volumes of the same name in the output " +
        "directory are overwritten."
      }
      error={error ?? undefined}
      submitLabel={"Pack"}
      status={result ? <Alert severity={"success"}>Archives packed.</Alert> : null}
      result={result ? <ArchivesPackResult result={result} /> : null}
      onSubmit={onPackClicked}
    >
      <PathFormRow
        isDisabled={isLoading}
        label={"Source"}
        description={"Directory to pack, normally a gamedata root"}
        field={source}
      />

      <PathFormRow
        isDisabled={isLoading}
        label={"Output"}
        description={"Directory the volumes are written into"}
        field={destination}
      />

      <PathFormRow
        isDisabled={isLoading}
        label={"Configuration"}
        description={"Optional LTX selecting what to include. Without one the whole directory is packed"}
        field={config}
      />

      <FormRow label={"Name"} description={"Base name of the volumes, written as <name>.db"} controlId={"pack-name"}>
        <TextField
          id={"pack-name"}
          size={"small"}
          fullWidth
          disabled={isLoading}
          value={name}
          placeholder={DEFAULT_ARCHIVE_NAME}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setName(event.target.value)}
        />
      </FormRow>

      <FormRow
        label={"Store only"}
        description={"Skip compression entirely and store every file"}
        controlId={"pack-store"}
        isInline
      >
        <Switch
          id={"pack-store"}
          disabled={isLoading}
          checked={isStore}
          slotProps={{ input: { "aria-label": "Store every file without compressing" } }}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setIsStore(event.target.checked)}
        />
      </FormRow>

      <FormRow
        label={"Volume size"}
        description={`Megabytes before a new volume starts, up to ${VOLUME_SIZE_MAX_MEGABYTES}. Empty uses the maximum`}
        controlId={"pack-max-size"}
        error={maxSizeError}
      >
        <TextField
          id={"pack-max-size"}
          size={"small"}
          fullWidth
          disabled={isLoading}
          value={maxSize}
          type={"number"}
          placeholder={String(VOLUME_SIZE_MAX_MEGABYTES)}
          error={Boolean(maxSizeError)}
          slotProps={{ htmlInput: { min: 1, max: VOLUME_SIZE_MAX_MEGABYTES } }}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setMaxSize(event.target.value)}
        />
      </FormRow>
    </PickerForm>
  );
}
