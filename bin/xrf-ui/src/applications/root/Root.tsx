import { Button, ButtonGroup, Card, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { NavigationFooter } from "@/core/components/NavigationFooter";

export function Root(): ReactElement {
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
      <Grid direction={"row"} justifyContent={"center"} marginBottom={2} container item>
        <Typography>XRF development tools</Typography>
      </Grid>

      <Card sx={{ minWidth: 200 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/spawn_editor", { replace: true })}>Spawn editor</Button>
            <Button onClick={() => navigate("/archive_editor", { replace: true })}>Archive editor</Button>
            <Button onClick={() => navigate("/dialog_editor", { replace: true })}>Dialog editor</Button>
            <Button onClick={() => navigate("/icon_editor", { replace: true })}>Icon editor</Button>
          </ButtonGroup>
        </Grid>
      </Card>

      <NavigationFooter />
    </Grid>
  );
}
