import { Box, Button, FormHelperText, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback, useEffect } from "react";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { FilePickerInput, usePathState } from "@/lib/file_picker";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectEquipmentDDSPath, getProjectSystemLtxPath } from "@/lib/xrf_path";

export function IconsEditorEquipmentOpenForm({
  equipmentContext: { spriteImage, equipmentActions } = useManager(EquipmentManager),
  projectContext: { xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("equipment-editor-open");

  const [spritePath, setSpritePath, onSelectEquipmentPath] = usePathState({
    title: "Provide path to equipment_editor dds",
    filters: [{ name: "dds", extensions: ["dds"] }],
    isDisabled: spriteImage.isLoading,
  });

  const [systemLtxPath, setSystemLtxPath, onSelectSystemLtxPath] = usePathState({
    title: "Provide path to system.ltx",
    filters: [{ name: "ltx", extensions: ["ltx"] }],
    isDisabled: spriteImage.isLoading,
  });

  const onOpenEquipmentClicked = useCallback(() => {
    if (spritePath && systemLtxPath) {
      equipmentActions.open(spritePath, systemLtxPath);
    } else {
      log.info("Cannot open equipment_editor when have no provided paths:", { spritePath, systemLtxPath });
    }
  }, [spritePath, systemLtxPath]);

  useEffect(() => {
    if (xrfProjectPath) {
      getPathIfExists(getProjectEquipmentDDSPath(xrfProjectPath)).then((equipmentPath) => {
        setSpritePath(equipmentPath);
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
            value={systemLtxPath}
            disabled={spriteImage.isLoading}
            onClick={onSelectSystemLtxPath}
          />

          <FilePickerInput
            label={"Equipment sprite"}
            value={spritePath}
            disabled={spriteImage.isLoading}
            onClick={onSelectEquipmentPath}
          />
        </Box>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "center", width: "auto" }}>
          <Button
            disabled={spriteImage.isLoading || !spritePath || !systemLtxPath}
            variant={"contained"}
            onClick={onOpenEquipmentClicked}
          >
            Open
          </Button>
        </Box>
      </Grid>

      {spriteImage.error ? (
        <Box>
          <FormHelperText error>{String(spriteImage.error)}</FormHelperText>
        </Box>
      ) : null}

      <ApplicationBackButton disabled={false} path={"/icons_editor"} />
    </Box>
  );
}
