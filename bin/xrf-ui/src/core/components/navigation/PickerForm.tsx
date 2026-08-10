import { Alert, Box, LinearProgress, Stack, Typography } from "@mui/material";
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
 * screen of every editor, so it sets the impression before any workspace is reached.
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
    <EditorLayout toolbar={<EditorToolbar backPath={backPath} isBackDisabled={backDisabled} />}>
      <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", minHeight: 0 }}>
        <Box sx={{ height: 2, flexShrink: 0 }}>{isLoading ? <LinearProgress /> : null}</Box>

        <Box sx={{ width: "100%", flexGrow: 1, minHeight: 0, overflowY: "auto", padding: 3 }}>
          <Stack spacing={2} sx={{ width: "100%", maxWidth: FORM_WIDTH }}>
            {title ? <Typography variant={"subtitle1"}>{title}</Typography> : null}

            {children ? <Stack spacing={2}>{children}</Stack> : null}

            {error ? <Alert severity={"error"}>{String(error)}</Alert> : null}

            {actions ? <Stack spacing={1}>{actions}</Stack> : null}

            {status ? <Box>{status}</Box> : null}
          </Stack>

          {result ? <Box sx={{ marginTop: 3 }}>{result}</Box> : null}
        </Box>
      </Box>
    </EditorLayout>
  );
}
