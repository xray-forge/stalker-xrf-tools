import { orange } from "@mui/material/colors";
import { createTheme, Theme } from "@mui/material/styles";

export function createApplicationTheme(): Theme {
  return createTheme({
    cssVariables: {
      colorSchemeSelector: "data-color-scheme",
    },
    colorSchemes: {
      light: {
        palette: {
          primary: {
            main: orange[500],
          },
          secondary: {
            main: orange[200],
          },
        },
      },
      dark: {
        palette: {
          primary: {
            main: orange[500],
          },
          secondary: {
            main: orange[200],
          },
        },
      },
    },
    defaultColorScheme: "dark",
  });
}
