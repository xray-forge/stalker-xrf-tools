export const RADIUS = {
  sm: 6,
  md: 8,
  lg: 12,
} as const;

export const ACCENT = {
  primary: {
    main: { light: "#8a5e0c", dark: "#ffb51a" },
    contrastText: { light: "#ffffff", dark: "#241b06" },
  },
  secondary: {
    main: { light: "#1e6699", dark: "#60bcff" },
    contrastText: { light: "#ffffff", dark: "#062138" },
  },
} as const;

export const STATUS = {
  success: { main: { light: "#5a8f2e", dark: "#9ccc65" } }, // toxic green
  warning: { main: { light: "#c2641a", dark: "#f2933e" } }, // burnt orange
  error: { main: { light: "#b23b30", dark: "#e0564a" } }, // warm red
} as const;

export const SURFACE = {
  default: { light: "#eef0f2", dark: "#161719" },
  paper: { light: "#f7f8fa", dark: "#1e2023" },
} as const;

export const TEXT = {
  primary: { light: "rgba(0, 0, 0, 0.87)", dark: "#d7dadc" },
  secondary: { light: "rgba(0, 0, 0, 0.6)", dark: "#969b9f" },
} as const;

export const DIVIDER = {
  light: "rgba(20, 28, 35, 0.1)",
  dark: "rgba(220, 228, 235, 0.08)",
} as const;
