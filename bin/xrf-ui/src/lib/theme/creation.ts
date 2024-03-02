import { PaletteMode, Theme } from "@mui/material";
import { orange } from "@mui/material/colors";
import { createTheme } from "@mui/material/styles";

export function createApplicationTheme(mode: PaletteMode): Theme {
  return createTheme({
    palette: {
      mode: mode,
      primary: {
        main: orange[500],
      },
      secondary: {
        main: orange[200],
      },
    },
  });
}
