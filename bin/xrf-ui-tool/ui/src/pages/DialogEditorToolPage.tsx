import { Button, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationHeader } from "@/components/header/ApplicationHeader";

export function DialogEditorToolPage(): ReactElement {
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
      <ApplicationHeader />

      <Typography variant={"h4"}>Dialog editing page</Typography>

      <Button onClick={() => navigate("/", { replace: true })}>Go back</Button>
    </Grid>
  );
}
