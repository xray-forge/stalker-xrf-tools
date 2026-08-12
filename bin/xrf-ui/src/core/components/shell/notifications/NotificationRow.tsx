import { default as CheckCircleOutlineIcon } from "@mui/icons-material/CheckCircleOutlineOutlined";
import { default as ContentCopyIcon } from "@mui/icons-material/ContentCopy";
import { default as ErrorOutlineIcon } from "@mui/icons-material/ErrorOutlineOutlined";
import { default as ExpandLessIcon } from "@mui/icons-material/ExpandLess";
import { default as ExpandMoreIcon } from "@mui/icons-material/ExpandMore";
import { default as InfoOutlinedIcon } from "@mui/icons-material/InfoOutlined";
import { default as WarningAmberIcon } from "@mui/icons-material/WarningAmber";
import { Box, Collapse, IconButton, Tooltip, Typography } from "@mui/material";
import { format } from "date-fns";
import { ReactElement, ReactNode, useCallback, useState } from "react";

import { findApplicationToolById, IApplicationTool } from "@/core/components/shell/application-tools";
import { Nullable } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { INotification, TNotificationSeverity } from "@/lib/notifications";

const SEVERITY_ICONS: Record<TNotificationSeverity, ReactNode> = {
  error: <ErrorOutlineIcon fontSize={"small"} />,
  info: <InfoOutlinedIcon fontSize={"small"} />,
  success: <CheckCircleOutlineIcon fontSize={"small"} />,
  warning: <WarningAmberIcon fontSize={"small"} />,
};

const SEVERITY_COLORS: Record<TNotificationSeverity, string> = {
  error: "error.main",
  info: "info.main",
  success: "success.main",
  warning: "warning.main",
};

export interface INotificationRowProps {
  notification: INotification;
}

/**
 * One recorded outcome.
 *
 * `details` is collapsed by default: a stack or a path list is what makes a record useful once you are
 * already reading it, and what makes the log unscannable before that.
 */
export function NotificationRow({ notification }: INotificationRowProps): ReactElement {
  const log: Logger = useLogger("notification-row");

  const [isExpanded, setExpanded] = useState<boolean>(false);

  const tool: Nullable<IApplicationTool> = findApplicationToolById(notification.source);
  const createdAt: Date = new Date(notification.createdAt);

  const onCopyDetails = useCallback(() => {
    navigator.clipboard?.writeText(notification.details ?? "").catch((error: unknown) => {
      log.error("Failed to copy notification details:", error);
    });
  }, [log, notification.details]);

  return (
    <Box sx={{ paddingX: 1.5, paddingY: 1, borderBottom: 1, borderColor: "divider" }}>
      <Box sx={{ display: "flex", alignItems: "flex-start", gap: 1 }}>
        <Box sx={{ display: "flex", paddingTop: 0.25, color: SEVERITY_COLORS[notification.severity] }}>
          {SEVERITY_ICONS[notification.severity]}
        </Box>

        <Box sx={{ flexGrow: 1, minWidth: 0 }}>
          <Typography variant={"body2"} sx={{ overflowWrap: "anywhere" }}>
            {notification.title}
          </Typography>

          <Box sx={{ display: "flex", alignItems: "center", gap: 0.75 }}>
            <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
              {tool ? tool.label : notification.source}
            </Typography>

            <Tooltip describeChild title={format(createdAt, "yyyy-MM-dd HH:mm:ss")} placement={"left"}>
              <Typography variant={"caption"} sx={{ color: "text.secondary", opacity: 0.7 }}>
                {format(createdAt, "HH:mm:ss")}
              </Typography>
            </Tooltip>
          </Box>
        </Box>

        {notification.details ? (
          <IconButton
            aria-label={isExpanded ? "Hide details" : "Show details"}
            aria-pressed={isExpanded}
            size={"small"}
            onClick={() => setExpanded((it: boolean) => !it)}
          >
            {isExpanded ? <ExpandLessIcon fontSize={"small"} /> : <ExpandMoreIcon fontSize={"small"} />}
          </IconButton>
        ) : null}
      </Box>

      {notification.details ? (
        <Collapse in={isExpanded} unmountOnExit>
          <Box sx={{ display: "flex", alignItems: "flex-start", gap: 0.5, marginTop: 0.5 }}>
            <Typography
              variant={"caption"}
              sx={{
                flexGrow: 1,
                minWidth: 0,
                padding: 1,
                borderRadius: 1,
                fontFamily: "monospace",
                whiteSpace: "pre-wrap",
                overflowWrap: "anywhere",
                backgroundColor: "background.paper",
              }}
            >
              {notification.details}
            </Typography>

            <Tooltip describeChild title={"Copy details"} placement={"left"}>
              <IconButton aria-label={"Copy details"} size={"small"} onClick={onCopyDetails}>
                <ContentCopyIcon fontSize={"small"} />
              </IconButton>
            </Tooltip>
          </Box>
        </Collapse>
      ) : null}
    </Box>
  );
}
