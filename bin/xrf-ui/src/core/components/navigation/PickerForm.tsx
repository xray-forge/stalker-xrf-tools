import { Alert, Box, CircularProgress, Stack, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";

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

const FORM_WIDTH: number = 460;

/**
 * Shared layout for the editors' "open / pick a path / run a command" screens.
 *
 * A settings-style pane rather than a card floating in the middle of the window: this is the first
 * screen of every editor, so it sets the impression before any workspace is reached. Leaving happens
 * through the toolbar, so the form no longer carries a back button of its own.
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
  return (
    <EditorLayout toolbar={<EditorToolbar backPath={backDisabled ? undefined : backPath} />}>
      <Box sx={{ width: "100%", height: "100%", overflowY: "auto", padding: 3 }}>
        <Stack spacing={2} sx={{ width: "100%", maxWidth: FORM_WIDTH }}>
          {title ? <Typography variant={"subtitle1"}>{title}</Typography> : null}

          {children ? (
            <Stack spacing={2} sx={{ "& .MuiFormControl-root, & .MuiOutlinedInput-root": { width: "100%" } }}>
              {children}
            </Stack>
          ) : null}

          {error ? <Alert severity={"error"}>{String(error)}</Alert> : null}

          {isLoading ? (
            <Box sx={{ display: "flex" }}>
              <CircularProgress size={20} />
            </Box>
          ) : null}

          {actions ? <Stack spacing={1}>{actions}</Stack> : null}

          {status ? <Box>{status}</Box> : null}
        </Stack>

        {result ? <Box sx={{ marginTop: 3 }}>{result}</Box> : null}
      </Box>
    </EditorLayout>
  );
}
