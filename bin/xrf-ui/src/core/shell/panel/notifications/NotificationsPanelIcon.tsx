import { default as NotificationsIcon } from "@mui/icons-material/Notifications";
import { Badge } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ENotificationSeverity } from "@/core/notifications";
import { NotificationsService } from "@/core/notifications/services";
import { Nullable } from "@/lib/types/general";

const BADGE_COLORS: Record<ENotificationSeverity, "default" | "success" | "info" | "warning" | "error"> = {
  [ENotificationSeverity.DEV]: "default",
  [ENotificationSeverity.ERROR]: "error",
  [ENotificationSeverity.INFO]: "info",
  [ENotificationSeverity.SUCCESS]: "success",
  [ENotificationSeverity.WARNING]: "warning",
};

/**
 * The stripe icon, badged with what has not been read yet.
 */
export function NotificationsPanelIcon(): ReactElement {
  const notificationsService: NotificationsService = useInjection(NotificationsService);

  const severity: Nullable<ENotificationSeverity> = notificationsService.highestUnreadSeverity;

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
