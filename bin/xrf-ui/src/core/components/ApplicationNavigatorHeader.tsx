import { default as HelpIcon } from "@mui/icons-material/Help";
import { Grid, IconButton, Typography } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { ReactElement, useCallback } from "react";

interface IApplicationNavigatorHeader {
  title: string;
  helpLink: string;
}

export function ApplicationNavigatorHeader({ title, helpLink }: IApplicationNavigatorHeader): ReactElement {
  const onOpenLink = useCallback(() => {
    open(helpLink).catch(console.error);
  }, [helpLink]);

  return (
    <Grid direction={"row"} justifyContent={"center"} container item>
      <Typography variant={"h6"}>{title}</Typography>

      <IconButton size={"small"} sx={{ margin: "0 4px" }} onClick={onOpenLink}>
        <HelpIcon />
      </IconButton>
    </Grid>
  );
}
