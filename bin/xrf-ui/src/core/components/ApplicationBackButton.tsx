import { Button } from "@mui/material";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

interface IApplicationBackButtonProps {
  disabled?: boolean;
  path: string;
}

export function ApplicationBackButton({ disabled, path }: IApplicationBackButtonProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const onClick = useCallback(() => navigate(path, { replace: true }), [navigate, path]);

  return (
    <Button
      variant={"outlined"}
      disabled={disabled}
      sx={{ minWidth: 200, marginTop: 2, marginBottom: 2, flexShrink: 0 }}
      onClick={onClick}
    >
      Back
    </Button>
  );
}
