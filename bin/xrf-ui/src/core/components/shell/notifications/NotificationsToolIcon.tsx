import { default as NotificationsIcon } from "@mui/icons-material/Notifications";
import { Badge } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { NotificationsService } from "@/core/store/notifications";
import { Nullable } from "@/core/types/general";
import { TNotificationSeverity } from "@/lib/notifications";

const BADGE_COLORS: Record<TNotificationSeverity, "success" | "info" | "warning" | "error"> = {
  error: "error",
  info: "info",
  success: "success",
  warning: "warning",
};

/**
 * The stripe icon, badged with what has not been read yet.
 */
export function NotificationsToolIcon(): ReactElement {
  const notificationsService: NotificationsService = useInjection(NotificationsService);

  const severity: Nullable<TNotificationSeverity> = notificationsService.highestUnreadSeverity;

  return (
    <Badge
      badgeContent={notificationsService.unreadCount}
      color={severity ? BADGE_COLORS[severity] : "default"}
      max={99}
      overlap={"circular"}
    >
      <NotificationsIcon />
    </Badge>
  );
}
