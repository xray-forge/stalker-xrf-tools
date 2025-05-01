import { Button, ButtonGroup, Card, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export function NavigationError(): ReactElement {
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
      <Grid direction={"row"} justifyContent={"center"} marginBottom={2} container>
        <Typography>XRF navigation error</Typography>
      </Grid>

      <Card>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/", { replace: true })}>Go home</Button>
          </ButtonGroup>
        </Grid>
      </Card>
    </Grid>
  );
}
