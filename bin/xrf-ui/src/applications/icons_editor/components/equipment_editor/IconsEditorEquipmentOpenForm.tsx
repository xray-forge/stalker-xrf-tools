import { Button } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect } from "react";

import { EquipmentService } from "@/applications/icons_editor/store/equipment";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectEquipmentDDSPath, getProjectSystemLtxPath } from "@/lib/xrf_path";

export function IconsEditorEquipmentOpenForm(): ReactElement {
  const log: Logger = useLogger("equipment-editor-open");

  const projectService: ProjectService = useInjection(ProjectService);
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  const [spritePath, setSpritePath, onSelectEquipmentPath] = usePathState({
    title: "Provide path to equipment_editor dds",
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
      log.info("Cannot open equipment_editor when have no provided paths:", { spritePath, systemLtxPath });
    }
  }, [spritePath, systemLtxPath, equipmentService, log]);

  useEffect(() => {
    if (projectService.xrfProjectPath) {
      getPathIfExists(getProjectEquipmentDDSPath(projectService.xrfProjectPath)).then((equipmentPath) => {
        setSpritePath(equipmentPath);
      });

      getPathIfExists(getProjectSystemLtxPath(projectService.xrfProjectPath)).then((ltxPath) => {
        setSystemLtxPath(ltxPath);
      });
    }
  }, []);

  return (
    <PickerForm
      title={"Provide equipment details"}
      error={equipmentService.spriteImage.error ? String(equipmentService.spriteImage.error) : undefined}
      isLoading={equipmentService.spriteImage.isLoading}
      backPath={"/icons_editor"}
      actions={
        <Button
          fullWidth
          disabled={equipmentService.spriteImage.isLoading || !spritePath || !systemLtxPath}
          variant={"contained"}
          onClick={onOpenEquipmentClicked}
        >
          Open
        </Button>
      }
    >
      <FilePickerInput
        label={"System ltx"}
        value={systemLtxPath}
        disabled={equipmentService.spriteImage.isLoading}
        onClick={onSelectSystemLtxPath}
      />

      <FilePickerInput
        label={"Equipment sprite"}
        value={spritePath}
        disabled={equipmentService.spriteImage.isLoading}
        onClick={onSelectEquipmentPath}
      />
    </PickerForm>
  );
}
