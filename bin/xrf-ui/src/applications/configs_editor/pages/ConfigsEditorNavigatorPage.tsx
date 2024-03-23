import { Button, ButtonGroup, Card, Grid } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/NavigationFooter";

export function ConfigsEditorNavigatorPage(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"column"}
      container={true}
      width={"100%"}
      height={"100%"}
      gap={1}
    >
      <ApplicationNavigatorHeader
        title={"XRF development tools"}
        helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/config_editor.html"}
      />

      <Card sx={{ minWidth: 200 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/configs_editor/explorer", { replace: true })}>Explorer</Button>
            <Button onClick={() => navigate("/configs_editor/verifier", { replace: true })}>Verifier</Button>
            <Button onClick={() => navigate("/configs_editor/formatter", { replace: true })}>Formatter</Button>
            <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
          </ButtonGroup>
        </Grid>
      </Card>

      <NavigationFooter />
    </Grid>
  );
}
