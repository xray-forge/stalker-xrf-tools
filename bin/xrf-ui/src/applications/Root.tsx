import { Box, Button, ButtonGroup, Card } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/footer/NavigationFooter";

export function Root(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column",
        width: "100%",
        height: "100%",
        gap: 1,
      }}
    >
      <ApplicationNavigatorHeader
        title={"XRF development tools"}
        helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/app.html"}
      />

      <Card sx={{ minWidth: 200 }}>
        <Box sx={{ display: "flex", flexDirection: "column" }}>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/archives_editor", { replace: true })}>Archive editor</Button>
            <Button onClick={() => navigate("/dialog_editor", { replace: true })}>Dialog editor</Button>
            <Button onClick={() => navigate("/configs_editor", { replace: true })}>Configs editor</Button>
            <Button onClick={() => navigate("/exports_editor", { replace: true })}>Exports editor</Button>
            <Button onClick={() => navigate("/icons_editor", { replace: true })}>Icon editor</Button>
            <Button onClick={() => navigate("/spawn_editor", { replace: true })}>Spawn editor</Button>
            <Button onClick={() => navigate("/translations_editor", { replace: true })}>Translation editor</Button>
          </ButtonGroup>
        </Box>
      </Card>

      <NavigationFooter />
    </Box>
  );
}
