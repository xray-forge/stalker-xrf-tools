import { Button, ButtonGroup, Card, Grid } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/footer/NavigationFooter";

export function IconsEditorNavigatorPage(): ReactElement {
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
        title={"XRF icons editor"}
        helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/icon_editor.html"}
      />

      <Card sx={{ minWidth: 200 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/icons_editor/icons_equipment", { replace: true })}>Equipment</Button>
            <Button onClick={() => navigate("/icons_editor/icons_description", { replace: true })}>Description</Button>
            <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
          </ButtonGroup>
        </Grid>
      </Card>

      <NavigationFooter />
    </Grid>
  );
}
