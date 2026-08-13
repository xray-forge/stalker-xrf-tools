import { default as ExpandLessIcon } from "@mui/icons-material/ExpandLess";
import { default as ExpandMoreIcon } from "@mui/icons-material/ExpandMore";
import { Alert, Box, Button, IconButton, LinearProgress, Stack, Tooltip, Typography } from "@mui/material";
import { FormEvent, KeyboardEvent, ReactElement, ReactNode, useCallback, useEffect, useRef, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/shell/EditorBusyContext";

const FORM_WIDTH: number = 460;

export interface IPickerFormProps {
  title?: ReactNode;
  /** The parameter rows. */
  children?: ReactNode;
  submitLabel?: string;
  isSubmitDisabled?: boolean;
  /** Follow-up actions shown beside the primary one, such as opening an output directory. */
  secondaryActions?: ReactNode;
  isLoading?: boolean;
  error?: ReactNode;
  status?: ReactNode;
  result?: ReactNode;
  onSubmit?: () => void;
}

/**
 * Shared layout for the editors' "pick some paths, run a command, read the output" screens.
 */
export function PickerForm({
  title,
  children,
  submitLabel,
  isSubmitDisabled,
  onSubmit,
  secondaryActions,
  error,
  isLoading,
  status,
  result,
}: IPickerFormProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  // Blocks navigation away from a running command, not just this form controls.
  useEditorBusy(Boolean(isLoading));

  const parametersRef = useRef<HTMLDivElement>(null);
  const [isCollapsed, setCollapsed] = useState<boolean>(false);

  const onFormSubmit = useCallback(
    (event: FormEvent) => {
      event.preventDefault();

      if (onSubmit && !isSubmitDisabled && !isLoading) {
        onSubmit();
      }
    },
    [isSubmitDisabled, isLoading, onSubmit]
  );

  const onLeave = useCallback(() => navigate("/", { replace: true }), [navigate]);

  const onFormKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Escape leaves the same way the button does, unless a command is still running.
      if (event.key === "Escape" && !isLoading) {
        onLeave();
      }
    },
    [isLoading, onLeave]
  );

  // Land on the first thing still to fill in, rather than making the user click into the form.
  useEffect(() => {
    const inputs: Array<HTMLInputElement> = Array.from(parametersRef.current?.querySelectorAll("input") ?? []);

    inputs.find((input) => !input.value)?.focus();
  }, []);

  return (
    <EditorLayout toolbar={<EditorToolbar />}>
      <Box
        component={"form"}
        noValidate={true}
        sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", minHeight: 0 }}
        onSubmit={onFormSubmit}
        onKeyDown={onFormKeyDown}
      >
        <Box sx={{ height: 2, flexShrink: 0 }}>{isLoading ? <LinearProgress /> : null}</Box>

        <Box
          ref={parametersRef}
          sx={{
            flexShrink: 0,
            paddingX: 3,
            paddingTop: 2,
            paddingBottom: isCollapsed ? 2 : 3,
            borderBottom: result ? 1 : 0,
            borderColor: "divider",
          }}
        >
          <Box sx={{ display: "flex", alignItems: "center", gap: 1, maxWidth: FORM_WIDTH }}>
            {title ? (
              <Typography variant={"subtitle1"} sx={{ flexGrow: 1 }}>
                {title}
              </Typography>
            ) : null}

            {/* Collapsing is only offered once there are results competing for the space. */}
            {result ? (
              <Tooltip title={isCollapsed ? "Show parameters" : "Hide parameters"}>
                <IconButton
                  aria-label={isCollapsed ? "Show parameters" : "Hide parameters"}
                  onClick={() => setCollapsed((it) => !it)}
                >
                  {isCollapsed ? <ExpandMoreIcon /> : <ExpandLessIcon />}
                </IconButton>
              </Tooltip>
            ) : null}
          </Box>

          {isCollapsed ? null : (
            <Stack spacing={2} sx={{ width: "100%", maxWidth: FORM_WIDTH, marginTop: 2 }}>
              {children}

              {error ? <Alert severity={"error"}>{String(error)}</Alert> : null}

              {status ? <Box>{status}</Box> : null}
            </Stack>
          )}
        </Box>

        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            flexGrow: 1,
            minHeight: 0,
            overflow: "hidden",
            paddingX: result ? 3 : 0,
            paddingY: result ? 2 : 0,
          }}
        >
          {result}
        </Box>

        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            gap: 1,
            flexShrink: 0,
            paddingX: 3,
            paddingY: 1.5,
            borderTop: 1,
            borderColor: "divider",
          }}
        >
          <Button type={"button"} color={"inherit"} disabled={isLoading} onClick={onLeave}>
            Back
          </Button>

          <Box sx={{ flexGrow: 1 }} />

          {secondaryActions}

          {submitLabel ? (
            <Button type={"submit"} variant={"contained"} disabled={isSubmitDisabled || isLoading}>
              {submitLabel}
            </Button>
          ) : null}
        </Box>
      </Box>
    </EditorLayout>
  );
}
