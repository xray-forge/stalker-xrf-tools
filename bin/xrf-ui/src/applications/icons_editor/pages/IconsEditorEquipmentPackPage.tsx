import { Box, Button, FormHelperText, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useState } from "react";

import { EquipmentPackResult } from "@/applications/icons_editor/components/equipment_pack/EquipmentPackResult";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { IPackEquipmentResult } from "@/lib/icons";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger, useLogger } from "@/lib/logging";
import {
  getPathIfExists,
  getProjectEquipmentDDSPath,
  getProjectEquipmentSourcePath,
  getProjectSystemLtxPath,
} from "@/lib/xrf_path";

export function IconsEditorEquipmentPackPage(): ReactElement {
  const log: Logger = useLogger("equipment-editor-pack");

  const { packEquipmentSprite } = useInjection(EquipmentManager);
  const { xrfProjectPath } = useInjection(ProjectManager);

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

        const packResult: IPackEquipmentResult = await packEquipmentSprite(
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
  }, [inputIconsPath, outputSpritePath, systemLtxPath, log, packEquipmentSprite]);

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
    <Box
      sx={{
        display: "flex",
        justifyContent: "safe center",
        alignItems: "safe center",
        flexDirection: "column",
        flexWrap: "nowrap",
        width: "100%",
        height: "100%",
        padding: 4,
      }}
    >
      <Grid container sx={{ justifyContent: "center", flexShrink: 0, marginBottom: 2 }}>
        <Typography>Provide equipment details</Typography>
      </Grid>

      <Grid container sx={{ justifyContent: "center", width: "auto", marginBottom: 2 }}>
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            width: "auto",
            marginRight: 1,
            gap: 2,
          }}
        >
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
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button
            disabled={!inputIconsPath || !outputSpritePath || !systemLtxPath || result.isLoading}
            variant={"contained"}
            onClick={onPackEquipmentClicked}
          >
            Pack
          </Button>
        </Box>
      </Grid>

      {result.error ? (
        <Box>
          <FormHelperText error>{String(result.error)}</FormHelperText>
        </Box>
      ) : null}

      {result.value ? <EquipmentPackResult result={result.value} /> : null}

      <ApplicationBackButton disabled={result.isLoading} path={"/icons_editor"} />
    </Box>
  );
}
