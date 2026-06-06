import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { Grid, IconButton } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { SettingsModalButton } from "@/core/components/settings/SettingsModalButton";
import { ThemeService } from "@/core/store/theme";

interface INavigationFooterProps {
  isWithSettings?: boolean;
}

export function NavigationFooter({ isWithSettings = true }: INavigationFooterProps): ReactElement {
  const { toggleTheme } = useInjection(ThemeService);

  const onOpenGithubLink = useCallback(() => {
    open("https://github.com/xray-forge/stalker-xrf-tools").catch(console.error);
  }, []);

  return (
    <Grid container sx={{ justifyContent: "center" }}>
      <IconButton onClick={onOpenGithubLink}>
        <GitHubIcon />
      </IconButton>

      <IconButton onClick={toggleTheme}>
        <LightModeIcon />
      </IconButton>

      {isWithSettings ? <SettingsModalButton /> : null}
    </Grid>
  );
}
