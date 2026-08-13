import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { EquipmentPackResult } from "@/applications/equipment-icons-pack/components/EquipmentPackResult";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { EApplicationId } from "@/core/router/application";
import { ProjectService } from "@/core/services/project";
import { Nullable } from "@/core/types/general";
import { FilePickerInput, usePathState } from "@/lib/file-picker";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger, useLogger } from "@/lib/logging";
import { ENotificationSeverity, TNotify, useNotify } from "@/lib/notifications";
import { useMountEffect } from "@/lib/react";
import { EquipmentService, IPackEquipmentResult } from "@/lib/xrf/icons";
import {
  getPathIfExists,
  getProjectEquipmentDDSPath,
  getProjectEquipmentSourcePath,
  getProjectSystemLtxPath,
} from "@/lib/xrf/project-path";

export function EquipmentIconsPackApplication(): ReactElement {
  const log: Logger = useLogger("equipment-editor-pack");
  const notify: TNotify = useNotify();

  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [result, setResult] = useState<Loadable<Nullable<IPackEquipmentResult>>>(() => createLoadable(null));

  const [inputIconsPath, setInputIconsPath, onSelectInputIconsPath] = usePathState({
    title: "Provide path to resulting equipment-editor dds",
    filters: [{ name: "dds", extensions: ["dds"] }],
    isDisabled: result.isLoading,
  });

  const [outputSpritePath, setOutputSpritePath, onSelectOutputSpritePath] = usePathState({
    title: "Provide path to source icons",
    isDirectory: true,
    isDisabled: result.isLoading,
  });

  const [systemLtxPath, setSystemLtxPath, onSelectSystemLtxPath] = usePathState({
    title: "Provide path to system.ltx",
    filters: [{ name: "ltx", extensions: ["ltx"] }],
    isDisabled: result.isLoading,
  });

  const onPackEquipmentClicked = useCallback(async () => {
    if (inputIconsPath && outputSpritePath && systemLtxPath) {
      try {
        setResult(createLoadable(null, true));

        const packResult: IPackEquipmentResult = await equipmentService.packEquipmentSprite(
          inputIconsPath,
          outputSpritePath,
          systemLtxPath
        );

        setResult(createLoadable(packResult));

        notify({
          details: outputSpritePath,
          severity: ENotificationSeverity.SUCCESS,
          source: EApplicationId.EQUIPMENT_ICONS_PACK,
          title: "Packed equipment sprite",
        });
      } catch (error) {
        log.error("Failed to pack equipment-editor:", error);

        setResult(createLoadable(null, false, error instanceof Error ? error : new Error(String(error))));

        notify({
          details: `${outputSpritePath}\n${String(error)}`,
          severity: ENotificationSeverity.ERROR,
          source: EApplicationId.EQUIPMENT_ICONS_PACK,
          title: "Could not pack equipment sprite",
        });
      }
    } else {
      log.info("Cannot open equipment-editor when have no provided paths:", {
        spritePath: outputSpritePath,
        systemLtxPath,
      });
    }
  }, [inputIconsPath, outputSpritePath, systemLtxPath, equipmentService, log, notify]);

  // Prefills from the project once: after mount these fields belong to whoever is typing in them.
  useMountEffect(() => {
    if (projectService.xrfProjectPath) {
      getProjectEquipmentDDSPath(projectService.xrfProjectPath).then((outputPath) => {
        setOutputSpritePath(outputPath);
      });

      getPathIfExists(getProjectEquipmentSourcePath(projectService.xrfProjectPath)).then((sourcePath) => {
        setInputIconsPath(sourcePath);
      });

      getPathIfExists(getProjectSystemLtxPath(projectService.xrfProjectPath)).then((ltxPath) => {
        setSystemLtxPath(ltxPath);
      });
    }
  });

  return (
    <PickerForm
      isLoading={result.isLoading}
      isSubmitDisabled={!inputIconsPath || !outputSpritePath || !systemLtxPath || result.isLoading}
      title={"Provide equipment details"}
      error={result.error ? String(result.error) : undefined}
      submitLabel={"Pack"}
      result={result.value ? <EquipmentPackResult result={result.value} /> : null}
      onSubmit={onPackEquipmentClicked}
    >
      <FilePickerInput
        isDisabled={result.isLoading}
        label={"System ltx"}
        value={systemLtxPath || ""}
        onSelect={onSelectSystemLtxPath}
      />

      <FilePickerInput
        isDisabled={result.isLoading}
        label={"Input icons directory"}
        value={inputIconsPath}
        onSelect={onSelectInputIconsPath}
      />

      <FilePickerInput
        isDisabled={result.isLoading}
        label={"Output equipment-editor sprite"}
        value={outputSpritePath}
        onSelect={onSelectOutputSpritePath}
      />
    </PickerForm>
  );
}
