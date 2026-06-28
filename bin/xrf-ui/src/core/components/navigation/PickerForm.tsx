import { Alert, Box, Button, Card, CircularProgress, Divider, Stack, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { CenteredColumn } from "@/core/components/layout/CenteredColumn";

export interface IPickerFormProps {
  isLoading?: boolean;
  title?: ReactNode;
  children?: ReactNode;
  actions?: ReactNode;
  error?: ReactNode;
  status?: ReactNode;
  result?: ReactNode;
  backPath?: string;
  backDisabled?: boolean;
}

/**
 * Shared layout for the editors' "open / pick a path / run a command" screens.
 */
export function PickerForm({
  title,
  children,
  actions,
  error,
  isLoading,
  status,
  result,
  backPath,
  backDisabled,
}: IPickerFormProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <CenteredColumn
      sx={{
        padding: 4,
        overflow: "auto",
        justifyContent: result ? "flex-start" : "center",
      }}
    >
      <Card sx={{ width: "100%", minWidth: 280, maxWidth: 400, flexShrink: 0 }}>
        <Stack spacing={2} sx={{ padding: 2.5 }}>
          {title ? (
            <Typography variant={"subtitle1"} align={"center"}>
              {title}
            </Typography>
          ) : null}

          {children ? (
            <Stack spacing={2} sx={{ "& .MuiFormControl-root, & .MuiOutlinedInput-root": { width: "100%" } }}>
              {children}
            </Stack>
          ) : null}

          {error ? <Alert severity={"error"}>{String(error)}</Alert> : null}

          {isLoading ? (
            <Box sx={{ display: "flex", justifyContent: "center" }}>
              <CircularProgress size={24} />
            </Box>
          ) : null}

          {actions ? <Stack spacing={1}>{actions}</Stack> : null}

          {status ? <Box>{status}</Box> : null}
        </Stack>

        {backPath ? (
          <>
            <Divider />
            <Button
              fullWidth
              disabled={backDisabled}
              sx={{ borderRadius: 0, paddingY: 1.25 }}
              onClick={() => navigate(backPath, { replace: true })}
            >
              Back
            </Button>
          </>
        ) : null}
      </Card>

      {result ? (
        <Box sx={{ width: "100%", display: "flex", justifyContent: "center", marginTop: 2 }}>{result}</Box>
      ) : null}
    </CenteredColumn>
  );
}
