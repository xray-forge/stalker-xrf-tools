import { default as LightModeIcon } from "@mui/icons-material/LightMode";
import { AppBar, Grid, IconButton } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { ThemeManager } from "@/store/theme";

export function ApplicationHeader({ themeContext: { themeActions } = useManager(ThemeManager) }): ReactElement {
  return (
    <AppBar sx={{ height: 56 }}>
      <Grid height={"100%"} alignItems={"center"} justifyContent={"space-between"} padding={"0 8px"} container>
        <Grid>todo</Grid>

        <Grid>
          <IconButton onClick={themeActions.toggleTheme}>
            <LightModeIcon />
          </IconButton>
        </Grid>
      </Grid>
    </AppBar>
  );
}
