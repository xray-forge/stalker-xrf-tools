import { Box } from "@mui/material";
import { ReactElement, ReactNode, useCallback } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { EditorToolbarCrumb } from "@/core/components/editor/EditorToolbarCrumb";
import { EditorToolbarPathSeparator } from "@/core/components/editor/EditorToolbarPathSeparator";
import { useIsEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { IApplicationDescriptor } from "@/core/router/application";
import { findApplication } from "@/core/router/applications";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IEditorToolbarProps extends BaseComponentProps {
  /** Overrides the application name resolved from the route. Rarely needed. */
  title?: string;
  /** The open document, as the last breadcrumb segment. Counts and state belong in the status bar. */
  subtitle?: ReactNode;
  actions?: ReactNode;
  /** Closes what is open. Reached through the application's own breadcrumb segment. */
  onBack?: () => void;
}

/**
 * The active application's row inside the window caption.
 */
export function EditorToolbar({
  "data-testid": dataTestId = "editor-toolbar",
  id = "editor-toolbar",
  className,
  title,
  subtitle,
  actions,
  onBack,
}: IEditorToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const isBusy: boolean = useIsEditorBusy();
  const application: Nullable<IApplicationDescriptor> = findApplication(pathname);

  const label: Nullable<string> = title ?? application?.label ?? null;

  const onGoHome = useCallback(() => navigate("/", { replace: true }), [navigate]);

  return (
    <Box
      data-testid={dataTestId}
      id={id}
      className={className}
      sx={{ display: "flex", alignItems: "center", gap: 0.75, width: "100%", height: "100%", minWidth: 0 }}
    >
      <EditorToolbarCrumb label={"XRF"} isDisabled={isBusy} hint={"Back to all applications"} onClick={onGoHome} />

      {label ? (
        <>
          <EditorToolbarPathSeparator />

          <EditorToolbarCrumb
            label={label}
            isDisabled={isBusy}
            accessibleName={onBack ? `Close ${label}` : undefined}
            hint={onBack ? "Close what is open" : undefined}
            onClick={onBack}
          />
        </>
      ) : null}

      {subtitle ? (
        <>
          <EditorToolbarPathSeparator />

          <Box
            sx={{
              direction: "rtl",
              textAlign: "left",
              minWidth: 0,
              overflow: "hidden",
              whiteSpace: "nowrap",
              textOverflow: "ellipsis",
              fontSize: "0.75rem",
              opacity: 0.7,
              "& > *": { direction: "ltr" },
            }}
          >
            {subtitle}
          </Box>
        </>
      ) : null}

      <Box sx={{ flexGrow: 1, minWidth: 8 }} />

      {actions ? (
        <>
          <Box
            sx={{
              display: "flex",
              alignItems: "center",
              flexShrink: 0,
              "& .MuiIconButton-root": { width: 24, height: 24, padding: 0 },
              "& .MuiSvgIcon-root": { fontSize: 16 },
            }}
          >
            {actions}
          </Box>

          <Box
            aria-hidden={true}
            sx={{ width: "1px", height: 18, marginLeft: 0.5, flexShrink: 0, backgroundColor: "divider" }}
          />
        </>
      ) : null}
    </Box>
  );
}
