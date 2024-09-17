import { Button, ButtonGroup, Card, Grid } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/footer/NavigationFooter";
import { IArchiveUnpackResult } from "@/lib/archive";
import { ETranslationsEditorCommand } from "@/lib/ipc";

export function TranslationsEditorNavigatorPage(): ReactElement {
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
        title={"XRF translations editor"}
        helpLink={"https://xray-forge.github.io/stalker-xrf-book/tools/app/translations_editor.html"}
      />

      <Card sx={{ minWidth: 200 }}>
        <Grid direction={"column"} container>
          <ButtonGroup orientation={"vertical"}>
            <Button
              onClick={async () => {
                const result: IArchiveUnpackResult = await invoke(
                  ETranslationsEditorCommand.READ_TRANSLATIONS_PROJECT,
                  {
                    path: "C:\\Projects\\stalker-xrf-engine\\src\\engine\\translations",
                  }
                );

                console.warn("re", result);
              }}
            >
              Test
            </Button>
            <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
          </ButtonGroup>
        </Grid>
      </Card>

      <NavigationFooter />
    </Grid>
  );
}
