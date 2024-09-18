import { Button, FormHelperText, Grid, Typography } from "@mui/material";
import { createLoadable, Loadable, useManager } from "dreamstate";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { EquipmentPackResult } from "@/applications/icons_editor/components/equipment_pack/EquipmentPackResult";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { IPackEquipmentResult } from "@/lib/icons";
import { Logger, useLogger } from "@/lib/logging";
import {
  getPathIfExists,
  getProjectEquipmentDDSPath,
  getProjectEquipmentSourcePath,
  getProjectSystemLtxPath,
} from "@/lib/xrf_path";

export function IconsEditorEquipmentPackPage({
  equipmentContext: { equipmentActions } = useManager(EquipmentManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("equipment-editor-pack");

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

        const packResult: IPackEquipmentResult = await equipmentActions.pack(
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
  }, [inputIconsPath, outputSpritePath, systemLtxPath, log]);

  useEffect(() => {
    if (xrfProjectPath) {
      getProjectEquipmentDDSPath(xrfProjectPath).then((outputPath) => {
        setOutputSpritePath(outputPath);
      });

      getPathIfExists(getProjectEquipmentSourcePath(xrfProjectPath)).then((sourcePath) => {
        setInputIconsPath(sourcePath);
      });

      getPathIfExists(getProjectSystemLtxPath(xrfProjectPath)).then((ltxPath) => {
        setSystemLtxPath(ltxPath);
      });
    }
  }, []);

  return (
    <Grid
      justifyContent={"safe center"}
      alignItems={"safe center"}
      direction={"column"}
      flexWrap={"nowrap"}
      container={true}
      width={"100%"}
      height={"100%"}
      padding={4}
    >
      <Grid direction={"row"} justifyContent={"center"} flexShrink={0} marginBottom={2} container item>
        <Typography>Provide equipment details</Typography>
      </Grid>

      <Grid direction={"row"} justifyContent={"center"} width={"auto"} marginBottom={2} container>
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} gap={2} container>
          <FilePickerInput
            label={"System ltx"}
            value={systemLtxPath || ""}
            disabled={result.isLoading}
            onClick={onSelectSystemLtxPath}
          />

          <FilePickerInput
            label={"Input icons directory"}
            value={inputIconsPath}
            disabled={result.isLoading}
            onClick={onSelectInputIconsPath}
          />

          <FilePickerInput
            label={"Output equipment_editor sprite"}
            value={outputSpritePath}
            disabled={result.isLoading}
            onClick={onSelectOutputSpritePath}
          />
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button
            disabled={!inputIconsPath || !outputSpritePath || !systemLtxPath || result.isLoading}
            variant={"contained"}
            onClick={onPackEquipmentClicked}
          >
            Pack
          </Button>
        </Grid>
      </Grid>

      {result.error ? (
        <Grid>
          <FormHelperText error>{String(result.error)}</FormHelperText>
        </Grid>
      ) : null}

      {result.value ? <EquipmentPackResult result={result.value} /> : null}

      <ApplicationBackButton disabled={result.isLoading} path={"/icons_editor"} />
    </Grid>
  );
}
