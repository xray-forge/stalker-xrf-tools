import { Box, Divider, ToggleButton, ToggleButtonGroup, Typography } from "@mui/material";
import { useColorScheme } from "@mui/material/styles";
import { open } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { SettingsPathField } from "@/core/components/settings/SettingsPathField";
import { ProjectService } from "@/core/store/project";
import { Optional } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { getProjectConfigsPath } from "@/lib/xrf_path";

type ColorSchemeMode = "light" | "dark" | "system";

export function SettingsForm(): ReactElement {
  const log: Logger = useLogger("settings-modal");

  const projectService: ProjectService = useInjection(ProjectService);
  const { mode, setMode } = useColorScheme();

  const onSelectProjectPath = useCallback(async () => {
    const newXrfProjectPath: Optional<string> = (await open({
      title: "Provide path to xrf project",
      directory: true,
    })) as Optional<string>;

    if (newXrfProjectPath) {
      log.info("Selected new project path:", newXrfProjectPath);

      projectService.setXrfProjectPath(newXrfProjectPath);

      // Try to auto-guess configs folder from xrf directory.
      if (!projectService.xrfConfigsPath) {
        const newXrfConfigsPath: string = await getProjectConfigsPath(newXrfProjectPath);

        if (await exists(newXrfConfigsPath)) {
          log.info("Automatically selected new configs path:", newXrfConfigsPath);
          projectService.setXrfConfigsPath(newXrfConfigsPath);
        }
      }
    }
  }, [log, projectService]);

  const onSelectConfigsPath = useCallback(async () => {
    const newXrfConfigsPath: Optional<string> = (await open({
      title: "Provide path to xrf configs",
      directory: true,
    })) as Optional<string>;

    if (newXrfConfigsPath) {
      log.info("Selected new configs path:", newXrfConfigsPath);

      projectService.setXrfConfigsPath(newXrfConfigsPath);
    }
  }, [log, projectService]);

  const onClearProjectPath = useCallback(() => projectService.setXrfProjectPath(null), [projectService]);
  const onClearConfigsPath = useCallback(() => projectService.setXrfConfigsPath(null), [projectService]);

  const onChangeMode = useCallback(
    (_: unknown, value: Optional<ColorSchemeMode>) => {
      if (value) {
        setMode(value);
      }
    },
    [setMode]
  );

  return (
    <Box sx={{ display: "flex", flexDirection: "column", gap: 3 }}>
      <Box>
        <Typography variant={"subtitle2"}>Appearance</Typography>

        <Typography variant={"caption"} sx={{ display: "block", color: "text.secondary", marginBottom: 1 }}>
          Follow the system theme, or pin the application to one.
        </Typography>

        <ToggleButtonGroup exclusive size={"small"} value={mode ?? "system"} onChange={onChangeMode}>
          <ToggleButton value={"light"}>Light</ToggleButton>
          <ToggleButton value={"dark"}>Dark</ToggleButton>
          <ToggleButton value={"system"}>System</ToggleButton>
        </ToggleButtonGroup>
      </Box>

      <Divider />

      <SettingsPathField
        label={"Project"}
        description={"Root of the xrf project. Most tools resolve their defaults from here."}
        value={projectService.xrfProjectPath}
        onSelect={onSelectProjectPath}
        onClear={onClearProjectPath}
      />

      <SettingsPathField
        label={"Configs"}
        description={"Directory holding LTX configs. Guessed from the project path when it is left empty."}
        value={projectService.xrfConfigsPath}
        onSelect={onSelectConfigsPath}
        onClear={onClearConfigsPath}
      />
    </Box>
  );
}
