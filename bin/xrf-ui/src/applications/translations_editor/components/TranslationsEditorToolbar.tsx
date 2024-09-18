import { AppBar, Toolbar, Typography } from "@mui/material";
import { ReactElement } from "react";

export function TranslationsEditorToolbar(): ReactElement {
  return (
    <AppBar position={"relative"}>
      <Toolbar variant={"dense"}>
        <Typography variant={"h6"} component={"div"}>
          todo
        </Typography>
      </Toolbar>
    </AppBar>
  );
}
