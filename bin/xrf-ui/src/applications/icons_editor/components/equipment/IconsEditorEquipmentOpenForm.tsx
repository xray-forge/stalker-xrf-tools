import { default as FolderIcon } from "@mui/icons-material/Folder";
import {
  Button,
  FormControl,
  FormHelperText,
  Grid,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
  Typography,
} from "@mui/material";
import { open } from "@tauri-apps/api/dialog";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useEffect, useState } from "react";

import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";
import { ProjectManager } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getPathIfExists, getProjectEquipmentDDSPath, getProjectSystemLtxPath } from "@/lib/xrf_path";

export function IconsEditorEquipmentOpenForm({
  equipmentContext: { spriteImage, equipmentActions } = useManager(EquipmentManager),
  projectContext: { projectActions, xrfProjectPath } = useManager(ProjectManager),
}): ReactElement {
  const log: Logger = useLogger("exports-open");

  const [spritePath, setSpritePath] = useState<Optional<string>>(null);
  const [systemLtxPath, setSystemLtxPath] = useState<Optional<string>>(null);

  const onSelectProjectPath = useCallback(async (event: MouseEvent<HTMLInputElement>) => {
    event.stopPropagation();
    event.preventDefault();

    const newXrfProjectPath: Optional<string> = (await open({
      title: "Provide path to xrf project",
      directory: true,
    })) as Optional<string>;

    if (newXrfProjectPath) {
      log.info("Selected new project path:", newXrfProjectPath);
      projectActions.setXrfProjectPath(newXrfProjectPath);
    }
  }, []);

  const onSelectEquipmentPath = useCallback(async (event: MouseEvent<HTMLInputElement>) => {
    event.stopPropagation();
    event.preventDefault();

    const newXrfProjectPath: Optional<string> = (await open({
      title: "Provide path to equipment dds",
      filters: [{ name: "dds", extensions: ["dds"] }],
      directory: false,
    })) as Optional<string>;

    if (newXrfProjectPath) {
      log.info("Selected new project path:", newXrfProjectPath);
      projectActions.setXrfProjectPath(newXrfProjectPath);
    }
  }, []);

  const onSelectProjectPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectProjectPath(event),
    [onSelectProjectPath]
  );

  const onSelectEquipmentPathClicked = useCallback(
    (event: MouseEvent<HTMLInputElement>) => onSelectEquipmentPath(event),
    [onSelectProjectPath]
  );

  const onOpenEquipmentClicked = useCallback(() => {
    if (spritePath && systemLtxPath) {
      equipmentActions.open(spritePath, systemLtxPath);
    } else {
      log.info("Cannot open equipment when have no provided paths:", { spritePath, systemLtxPath });
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
        <Grid direction={"column"} justifyContent={"center"} width={"auto"} marginRight={1} gap={1} container item>
          <Grid alignItems={"center"} justifyContent={"center"} container>
            <FormControl size={"small"} variant={"outlined"}>
              <InputLabel size={"small"}>System ltx</InputLabel>
              <OutlinedInput
                size={"small"}
                type={"text"}
                endAdornment={
                  <InputAdornment position={"end"} onClick={onSelectProjectPath}>
                    <IconButton edge={"end"}>
                      <FolderIcon />
                    </IconButton>
                  </InputAdornment>
                }
                label={"System ltx"}
                value={systemLtxPath || ""}
                readOnly
                onClick={onSelectProjectPathClicked}
              />
            </FormControl>
          </Grid>

          <Grid alignItems={"center"} justifyContent={"center"} container>
            <FormControl sx={{ m: "8px 0" }} size={"small"} variant={"outlined"}>
              <InputLabel size={"small"}>Equipment sprite</InputLabel>
              <OutlinedInput
                size={"small"}
                type={"text"}
                endAdornment={
                  <InputAdornment position={"end"} onClick={onSelectEquipmentPathClicked}>
                    <IconButton edge={"end"}>
                      <FolderIcon />
                    </IconButton>
                  </InputAdornment>
                }
                label={"Equipment sprite"}
                value={spritePath || ""}
                readOnly
                onClick={onSelectEquipmentPath}
              />
            </FormControl>
          </Grid>
        </Grid>

        <Grid direction={"column"} justifyContent={"center"} width={"auto"} container item>
          <Button
            disabled={spriteImage.isLoading || !spritePath || !systemLtxPath}
            variant={"contained"}
            onClick={onOpenEquipmentClicked}
          >
            Open
          </Button>
        </Grid>
      </Grid>

      {spriteImage.error ? (
        <Grid>
          <FormHelperText error>{String(spriteImage.error)}</FormHelperText>
        </Grid>
      ) : null}

      <ApplicationBackButton disabled={false} path={"/icons_editor"} />
    </Grid>
  );
}
