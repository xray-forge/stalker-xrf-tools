import { createTheme, PaletteOptions, Theme } from "@mui/material/styles";
// Type-only, side-effect import: it pulls in `@mui/x-data-grid`'s module augmentation,
// which registers the `MuiDataGrid` slot on MUI's `Components` type.
import type {} from "@mui/x-data-grid/themeAugmentation";

import { ACCENT, DIVIDER, RADIUS, STATUS, SURFACE, TEXT } from "@/lib/theme/tokens";

type ColorScheme = "light" | "dark";

/**
 * Maps the design tokens onto a MUI palette for one color scheme.
 */
function createColorSchemePalette(scheme: ColorScheme): PaletteOptions {
  return {
    primary: { main: ACCENT.primary.main[scheme], contrastText: ACCENT.primary.contrastText[scheme] },
    secondary: { main: ACCENT.secondary.main[scheme], contrastText: ACCENT.secondary.contrastText[scheme] },
    success: { main: STATUS.success.main[scheme] },
    warning: { main: STATUS.warning.main[scheme] },
    error: { main: STATUS.error.main[scheme] },
    background: { default: SURFACE.default[scheme], paper: SURFACE.paper[scheme] },
    text: { primary: TEXT.primary[scheme], secondary: TEXT.secondary[scheme] },
    divider: DIVIDER[scheme],
  };
}

export function createApplicationTheme(): Theme {
  return createTheme({
    cssVariables: {
      colorSchemeSelector: "data-color-scheme",
    },
    defaultColorScheme: "dark",
    shape: {
      borderRadius: RADIUS.md,
    },
    typography: {
      fontFamily: ["'Roboto'", "'Segoe UI'", "system-ui", "sans-serif"].join(", "),
      h5: { fontWeight: 600 },
      h6: { fontWeight: 600 },
      button: {
        textTransform: "none",
        fontWeight: 500,
      },
    },
    colorSchemes: {
      light: { palette: createColorSchemePalette("light") },
      dark: { palette: createColorSchemePalette("dark") },
    },
    components: {
      // Flat surfaces: MUI's dark elevation overlay tints `paper`.
      MuiPaper: {
        styleOverrides: {
          root: { backgroundImage: "none" },
        },
      },
      MuiCard: {
        defaultProps: { variant: "outlined" },
        styleOverrides: {
          root: ({ theme }) => ({
            borderRadius: RADIUS.lg,
            borderColor: theme.palette.divider,
          }),
        },
      },
      MuiButton: {
        defaultProps: { disableElevation: true },
      },
      MuiTextField: {
        defaultProps: { size: "small" },
      },
      MuiOutlinedInput: {
        styleOverrides: {
          root: { borderRadius: RADIUS.sm },
        },
      },
      MuiDataGrid: {
        defaultProps: {
          density: "compact",
          disableRowSelectionOnClick: true,
        },
        styleOverrides: {
          root: { border: "none" },
        },
      },
    },
  });
}
