import { default as DarkModeIcon } from "@mui/icons-material/DarkMode";
import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { Grid, IconButton } from "@mui/material";
import { useColorScheme } from "@mui/material/styles";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, useCallback } from "react";

import { SettingsModalButton } from "@/core/components/settings/SettingsModalButton";
import { Maybe } from "@/core/types/general";

interface INavigationFooterProps {
  isWithSettings?: boolean;
}

export function NavigationFooter({ isWithSettings = true }: INavigationFooterProps): ReactElement {
  const { mode, setMode, systemMode } = useColorScheme();

  const resolvedMode: Maybe<string> = mode === "system" ? systemMode : mode;
  const isLightMode: boolean = resolvedMode === "light";

  const onOpenGithubLink = useCallback(() => {
    open("https://github.com/xray-forge/stalker-xrf-tools").catch(console.error);
  }, []);

  const onToggleTheme = useCallback(() => {
    setMode(isLightMode ? "dark" : "light");
  }, [isLightMode, setMode]);

  return (
    <Grid container sx={{ justifyContent: "center" }}>
      <IconButton onClick={onOpenGithubLink}>
        <GitHubIcon />
      </IconButton>

      <IconButton onClick={onToggleTheme}>{isLightMode ? <DarkModeIcon /> : <LightModeIcon />}</IconButton>

      {isWithSettings ? <SettingsModalButton /> : null}
    </Grid>
  );
}
