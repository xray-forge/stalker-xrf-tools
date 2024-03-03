import { Button, ButtonGroup, Card, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { NavigationFooter } from "@/core/components/NavigationFooter";

export function ConfigsEditor(): ReactElement {
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
        <Typography>XRF configs editor</Typography>
      </Grid>

      <Card sx={{ minWidth: 200 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/configs_editor/todo", { replace: true })}>Editor</Button>
            <Button onClick={() => navigate("/configs_editor/todo", { replace: true })}>Validator</Button>
            <Button onClick={() => navigate("/configs_editor/todo", { replace: true })}>Formatter</Button>
            <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
          </ButtonGroup>
        </Grid>
      </Card>

      <NavigationFooter />
    </Grid>
  );
}
