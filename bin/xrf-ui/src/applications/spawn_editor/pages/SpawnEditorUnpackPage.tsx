import { Button, Grid } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export function SpawnEditorUnpackPage(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Grid>
      Spawn unpack
      <Button onClick={() => navigate("/", { replace: true })}>Back</Button>
    </Grid>
  );
}
