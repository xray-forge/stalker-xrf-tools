import { Button, ButtonGroup, Card, Grid } from "@mui/material";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

interface ISpawnBackButtonProps {
  disabled?: boolean;
  path: string;
}

export function ApplicationBackButton({ disabled, path }: ISpawnBackButtonProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const onClick = useCallback(() => navigate(path, { replace: true }), [navigate, path]);

  return (
    <Card sx={{ minWidth: 200, marginTop: 2, marginBottom: 2, flexShrink: 0 }}>
      <Grid direction={"column"} container>
        <ButtonGroup orientation={"vertical"}>
          <Button disabled={disabled} onClick={onClick}>
            Back
          </Button>
        </ButtonGroup>
      </Grid>
    </Card>
  );
}
