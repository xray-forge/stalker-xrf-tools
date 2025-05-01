import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { Grid, IconButton } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { useManager } from "dreamstate";
import { ReactElement, useCallback } from "react";

import { SettingsModalButton } from "@/core/components/settings/SettingsModalButton";
import { IThemeContext, ThemeManager } from "@/core/store/theme";

interface INavigationFooterProps {
  themeContext?: IThemeContext;
  isWithSettings?: boolean;
}

export function NavigationFooter({
  themeContext: { themeActions } = useManager(ThemeManager),
  isWithSettings = true,
}: INavigationFooterProps): ReactElement {
  const onOpenGithubLink = useCallback(() => {
    open("https://github.com/xray-forge/stalker-xrf-tools").catch(console.error);
  }, []);

  return (
    <Grid direction={"row"} justifyContent={"center"} container>
      <IconButton onClick={onOpenGithubLink}>
        <GitHubIcon />
      </IconButton>

      <IconButton onClick={themeActions.toggleTheme}>
        <LightModeIcon />
      </IconButton>

      {isWithSettings ? <SettingsModalButton /> : null}
    </Grid>
  );
}
