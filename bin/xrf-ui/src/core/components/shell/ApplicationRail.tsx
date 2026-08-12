import { default as DarkModeIcon } from "@mui/icons-material/DarkMode";
import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as HomeIcon } from "@mui/icons-material/Home";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { default as SettingsIcon } from "@mui/icons-material/Settings";
import { useColorScheme } from "@mui/material/styles";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, useCallback, useState } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { SettingsDialog } from "@/core/components/settings/SettingsDialog";
import { ApplicationPanelStripe } from "@/core/components/shell/ApplicationPanelStripe";
import { useIsEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { IEditorPanel } from "@/core/components/shell/EditorPanelsContext";
import { RailButton } from "@/core/components/shell/RailButton";
import { Maybe, Nullable } from "@/core/types/general";

export interface IApplicationRailProps {
  panels: Array<IEditorPanel>;
  activePanelId: Nullable<string>;
  onTogglePanel: (id: string) => void;
}

/**
 * The left edge: Home, the active application's navigation panels, then the window's own controls.
 *
 * It used to list every tool, so that moving between them was one click. That list moved to the home
 * page when tools became nineteen flat applications instead of eight categories - going somewhere else
 * is Home and then a card now, and this rail belongs to whatever application is open.
 */
export function ApplicationRail({ panels, activePanelId, onTogglePanel }: IApplicationRailProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();
  const { mode, setMode, systemMode } = useColorScheme();

  const [isSettingsOpen, setSettingsOpen] = useState(false);

  const isBusy: boolean = useIsEditorBusy();

  const resolvedMode: Maybe<string> = mode === "system" ? systemMode : mode;
  const isLightMode: boolean = resolvedMode === "light";

  const onOpenGithubLink = useCallback(() => {
    open("https://github.com/xray-forge/stalker-xrf-tools").catch(console.error);
  }, []);

  const onToggleTheme = useCallback(() => {
    setMode(isLightMode ? "dark" : "light");
  }, [isLightMode, setMode]);

  return (
    <ApplicationPanelStripe
      side={"left"}
      panels={panels}
      activePanelId={activePanelId}
      header={
        <RailButton
          /*
            Navigation is blocked while the active application is running a command. Leaving mid
            operation left it running against a screen nobody could see. Only the navigating control is
            blocked: the theme toggle, settings and the source link do not abandon anything.
          */
          isDisabled={isBusy}
          isSelected={pathname === "/"}
          label={"Home"}
          icon={<HomeIcon fontSize={"small"} />}
          onClick={() => navigate("/", { replace: true })}
        />
      }
      footer={
        <>
          <RailButton
            label={isLightMode ? "Dark theme" : "Light theme"}
            icon={isLightMode ? <DarkModeIcon fontSize={"small"} /> : <LightModeIcon fontSize={"small"} />}
            onClick={onToggleTheme}
          />

          <RailButton label={"Source on github"} icon={<GitHubIcon fontSize={"small"} />} onClick={onOpenGithubLink} />

          <RailButton
            label={"Settings"}
            icon={<SettingsIcon fontSize={"small"} />}
            onClick={() => setSettingsOpen(true)}
          />

          <SettingsDialog isOpen={isSettingsOpen} onClose={() => setSettingsOpen(false)} />
        </>
      }
      onTogglePanel={onTogglePanel}
    />
  );
}
