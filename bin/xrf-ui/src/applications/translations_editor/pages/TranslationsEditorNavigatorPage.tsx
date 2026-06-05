import { Box, Button, ButtonGroup, Card } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/footer/NavigationFooter";

export function TranslationsEditorNavigatorPage(): ReactElement {
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
        title={"XRF translations editor"}
        helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/translations_editor.html"}
      />

      <Card sx={{ minWidth: 200 }}>
        <Box sx={{ display: "flex", flexDirection: "column" }}>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/translations_editor/project", { replace: true })}>Open</Button>
            <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
          </ButtonGroup>
        </Box>
      </Card>

      <NavigationFooter />
    </Box>
  );
}
