import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { EquipmentPackResult } from "@/applications/icons_editor/components/equipment_pack/EquipmentPackResult";
import { EquipmentService } from "@/applications/icons_editor/store/equipment";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput, usePathState } from "@/lib/file-picker";
import { IPackEquipmentResult } from "@/lib/icons";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger, useLogger } from "@/lib/logging";
import {
  getPathIfExists,
  getProjectEquipmentDDSPath,
  getProjectEquipmentSourcePath,
  getProjectSystemLtxPath,
} from "@/lib/xrf-path";

export function IconsEditorEquipmentPackPage(): ReactElement {
  const log: Logger = useLogger("equipment-editor-pack");

  const equipmentService: EquipmentService = useInjection(EquipmentService);
  const projectService: ProjectService = useInjection(ProjectService);

  const [result, setResult] = useState<Loadable<Optional<IPackEquipmentResult>>>(() => createLoadable(null));

  const [inputIconsPath, setInputIconsPath, onSelectInputIconsPath] = usePathState({
    title: "Provide path to resulting equipment_editor dds",
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
      } catch (error) {
        log.error("Failed to pack equipment_editor:", error);

        setResult(createLoadable(null, false, error instanceof Error ? error : new Error(String(error))));
      }
    } else {
      log.info("Cannot open equipment_editor when have no provided paths:", {
        spritePath: outputSpritePath,
        systemLtxPath,
      });
    }
  }, [inputIconsPath, outputSpritePath, systemLtxPath, equipmentService, log]);

  useEffect(() => {
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
  }, []);

  return (
    <PickerForm
      title={"Provide equipment details"}
      error={result.error ? String(result.error) : undefined}
      isLoading={result.isLoading}
      backPath={"/icons_editor"}
      backDisabled={result.isLoading}
      submitLabel={"Pack"}
      isSubmitDisabled={!inputIconsPath || !outputSpritePath || !systemLtxPath || result.isLoading}
      onSubmit={onPackEquipmentClicked}
      result={result.value ? <EquipmentPackResult result={result.value} /> : null}
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
        label={"Output equipment_editor sprite"}
        value={outputSpritePath}
        onSelect={onSelectOutputSpritePath}
      />
    </PickerForm>
  );
}
