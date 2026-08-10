import { default as DarkModeIcon } from "@mui/icons-material/DarkMode";
import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as HomeIcon } from "@mui/icons-material/Home";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { default as SettingsIcon } from "@mui/icons-material/Settings";
import { Box, IconButton, Tooltip } from "@mui/material";
import { useColorScheme } from "@mui/material/styles";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, ReactNode, useCallback, useState } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { SettingsDialog } from "@/core/components/settings/SettingsDialog";
import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/applicationTools";
import { useIsEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { Maybe } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

interface IRailButtonProps {
  isSelected?: boolean;
  isDisabled?: boolean;
  label: string;
  icon: ReactNode;
  onClick: () => void;
}

function RailButton({ isSelected, isDisabled, label, icon, onClick }: IRailButtonProps): ReactElement {
  return (
    <Tooltip describeChild title={label} placement={"right"}>
      <span>
        <IconButton
          aria-label={label}
          disabled={isDisabled}
          sx={{
            width: 36,
            height: 36,
            borderRadius: 1,
            color: isSelected ? "primary.main" : "text.secondary",
            backgroundColor: isSelected ? "action.selected" : "transparent",
          }}
          onClick={onClick}
        >
          {icon}
        </IconButton>
      </span>
    </Tooltip>
  );
}

/**
 * Permanent tool rail.
 *
 * Replaces the launcher-and-back navigation with chrome that never goes away: every tool is one click
 * from every other tool, and there is no screen the window can be "inside" of.
 */
export function ApplicationRail(): ReactElement {
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
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 0.5,
        width: LAYOUT.railWidth,
        minWidth: LAYOUT.railWidth,
        paddingY: 1,
        borderRight: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
      }}
    >
      {}
      <RailButton
        /*
          Navigation is blocked while the active editor is running a command. Leaving mid-operation left
          it running against a screen nobody could see. Only the navigating controls are blocked: the
          theme toggle, settings and the source link do not abandon anything.
        */
        isDisabled={isBusy}
        isSelected={pathname === "/"}
        label={"Home"}
        icon={<HomeIcon />}
        onClick={() => navigate("/", { replace: true })}
      />

      <Box sx={{ width: 24, borderBottom: 1, borderColor: "divider", marginY: 0.5 }} />

      {APPLICATION_TOOLS.map((tool: IApplicationTool) => (
        <RailButton
          isSelected={pathname.startsWith(tool.path)}
          isDisabled={isBusy}
          key={tool.path}
          label={tool.label}
          icon={tool.icon}
          onClick={() => navigate(tool.path, { replace: true })}
        />
      ))}

      <Box sx={{ flexGrow: 1 }} />

      <RailButton
        label={isLightMode ? "Dark theme" : "Light theme"}
        icon={isLightMode ? <DarkModeIcon /> : <LightModeIcon />}
        onClick={onToggleTheme}
      />

      <RailButton label={"Source on github"} icon={<GitHubIcon />} onClick={onOpenGithubLink} />

      <RailButton label={"Settings"} icon={<SettingsIcon />} onClick={() => setSettingsOpen(true)} />

      <SettingsDialog isOpen={isSettingsOpen} onClose={() => setSettingsOpen(false)} />
    </Box>
  );
}
