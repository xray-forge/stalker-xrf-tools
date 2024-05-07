import { Button, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationBackButton } from "@/core/components/ApplicationBackButton";

export function IconsEditorEquipmentOpenPage(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Grid
      justifyContent={"safe center"}
      alignItems={"safe center"}
      direction={"column"}
      flexWrap={"nowrap"}
      container={true}
      width={"100%"}
      height={"100%"}
      padding={4}
    >
      <Grid direction={"row"} justifyContent={"center"} flexShrink={0} marginBottom={2} container item>
        <Typography>Provide equipment texture sprite to open</Typography>
      </Grid>

      <Button onClick={() => navigate("/icons_editor/icons_equipment/edit", { replace: true })}>Test open</Button>

      <ApplicationBackButton disabled={false} path={"/icons_editor"} />
    </Grid>
  );
}
