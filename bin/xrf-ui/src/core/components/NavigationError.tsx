import { Box, Button, ButtonGroup, Card, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export function NavigationError(): ReactElement {
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
      }}
    >
      <Grid container sx={{ justifyContent: "center", marginBottom: 2 }}>
        <Typography>XRF navigation error</Typography>
      </Grid>

      <Card>
        <Box sx={{ display: "flex", flexDirection: "column" }}>
          <ButtonGroup orientation={"vertical"}>
            <Button onClick={() => navigate("/", { replace: true })}>Go home</Button>
          </ButtonGroup>
        </Box>
      </Card>
    </Box>
  );
}
