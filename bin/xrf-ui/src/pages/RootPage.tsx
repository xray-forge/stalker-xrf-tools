import { Button, Grid } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export function RootPage(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
    >
      <Button onClick={() => navigate("/spawn_editor", { replace: true })}>Use spawn tool</Button>
      <Button onClick={() => navigate("/archive_editor", { replace: true })}>Use archive tool</Button>
      <Button onClick={() => navigate("/dialog_editor", { replace: true })}>Use dialog tool</Button>
    </Grid>
  );
}
