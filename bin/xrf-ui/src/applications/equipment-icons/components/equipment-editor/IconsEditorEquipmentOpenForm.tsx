import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput, usePathState } from "@/lib/file-picker";
import { EquipmentService } from "@/lib/icons";
import { Logger, useLogger } from "@/lib/logging";
import { useMountEffect } from "@/lib/react";
import { getPathIfExists, getProjectEquipmentDDSPath, getProjectSystemLtxPath } from "@/lib/xrf-path";

export function IconsEditorEquipmentOpenForm(): ReactElement {
  const log: Logger = useLogger("equipment-editor-open");

  const projectService: ProjectService = useInjection(ProjectService);
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  const [spritePath, setSpritePath, onSelectEquipmentPath] = usePathState({
    title: "Provide path to equipment-editor dds",
    filters: [{ name: "dds", extensions: ["dds"] }],
    isDisabled: equipmentService.spriteImage.isLoading,
  });

  const [systemLtxPath, setSystemLtxPath, onSelectSystemLtxPath] = usePathState({
    title: "Provide path to system.ltx",
    filters: [{ name: "ltx", extensions: ["ltx"] }],
    isDisabled: equipmentService.spriteImage.isLoading,
  });

  const onOpenEquipmentClicked = useCallback(() => {
    if (spritePath && systemLtxPath) {
      equipmentService.openEquipmentProject(spritePath, systemLtxPath);
    } else {
      log.info("Cannot open equipment-editor when have no provided paths:", { spritePath, systemLtxPath });
    }
  }, [spritePath, systemLtxPath, equipmentService, log]);

  // Prefills from the project once: after mount these fields belong to whoever is typing in them.
  useMountEffect(() => {
    if (projectService.xrfProjectPath) {
      getPathIfExists(getProjectEquipmentDDSPath(projectService.xrfProjectPath)).then((equipmentPath) => {
        setSpritePath(equipmentPath);
      });

      getPathIfExists(getProjectSystemLtxPath(projectService.xrfProjectPath)).then((ltxPath) => {
        setSystemLtxPath(ltxPath);
      });
    }
  });

  return (
    <PickerForm
      isLoading={equipmentService.spriteImage.isLoading}
      isSubmitDisabled={equipmentService.spriteImage.isLoading || !spritePath || !systemLtxPath}
      title={"Provide equipment details"}
      error={equipmentService.spriteImage.error ? String(equipmentService.spriteImage.error) : undefined}
      submitLabel={"Open"}
      onSubmit={onOpenEquipmentClicked}
    >
      <FilePickerInput
        isDisabled={equipmentService.spriteImage.isLoading}
        label={"System ltx"}
        value={systemLtxPath}
        onSelect={onSelectSystemLtxPath}
      />

      <FilePickerInput
        isDisabled={equipmentService.spriteImage.isLoading}
        label={"Equipment sprite"}
        value={spritePath}
        onSelect={onSelectEquipmentPath}
      />
    </PickerForm>
  );
}
