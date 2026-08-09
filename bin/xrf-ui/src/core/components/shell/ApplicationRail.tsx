import { default as DarkModeIcon } from "@mui/icons-material/DarkMode";
import { default as GitHubIcon } from "@mui/icons-material/GitHub";
import { default as HomeIcon } from "@mui/icons-material/Home";
import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { Box, IconButton, Tooltip } from "@mui/material";
import { useColorScheme } from "@mui/material/styles";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { SettingsModalButton } from "@/core/components/settings/SettingsModalButton";
import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/applicationTools";
import { Maybe } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

interface IRailButtonProps {
  label: string;
  icon: ReactNode;
  isSelected?: boolean;
  onClick: () => void;
}

function RailButton({ label, icon, isSelected, onClick }: IRailButtonProps): ReactElement {
  return (
    <Tooltip title={label} placement={"right"}>
      <IconButton
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
      <RailButton
        label={"Home"}
        icon={<HomeIcon />}
        isSelected={pathname === "/"}
        onClick={() => navigate("/", { replace: true })}
      />

      <Box sx={{ width: 24, borderBottom: 1, borderColor: "divider", marginY: 0.5 }} />

      {APPLICATION_TOOLS.map((tool: IApplicationTool) => (
        <RailButton
          key={tool.path}
          label={tool.label}
          icon={tool.icon}
          isSelected={pathname.startsWith(tool.path)}
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

      <SettingsModalButton />
    </Box>
  );
}
