import { Box, Button, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useEffect } from "react";

import { NotificationRow } from "@/core/components/shell/notifications/NotificationRow";
import { NotificationsService } from "@/core/store/notifications";
import { SettingsService } from "@/core/store/settings";
import { INotification } from "@/lib/notifications";

/**
 * The notification centre panel.
 *
 * Newest first, flat and unfiltered. The cap is what keeps it readable for now - filters and grouping
 * are worth adding once a real run proves the list is too long to scan, not before.
 */
export function NotificationsPanel(): ReactElement {
  const notificationsService: NotificationsService = useInjection(NotificationsService);
  const settingsService: SettingsService = useInjection(SettingsService);

  const notifications: Array<INotification> = settingsService.isDevModeEnabled
    ? notificationsService.allNotifications
    : notificationsService.notifications;
  const unreadCount: number = notificationsService.unreadCount;

  // Anything visible is read, including what arrives while the panel is open - otherwise the badge
  // counts records the user is looking at, and nothing can dismiss it.
  useEffect(() => {
    notificationsService.markAllRead();
  }, [notificationsService, unreadCount]);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", minHeight: 0, height: "100%" }}>
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 1,
          paddingX: 1.5,
          paddingY: 1,
          borderBottom: 1,
          borderColor: "divider",
        }}
      >
        <Typography variant={"subtitle2"}>Notifications</Typography>

        <Button disabled={!notifications.length} size={"small"} onClick={notificationsService.clear}>
          Clear all
        </Button>
      </Box>

      <Box sx={{ flexGrow: 1, minHeight: 0, overflowY: "auto" }}>
        {notifications.length ? (
          notifications.map((notification: INotification) => (
            <NotificationRow key={notification.id} notification={notification} />
          ))
        ) : (
          <Typography variant={"caption"} sx={{ display: "block", padding: 2, color: "text.secondary" }}>
            Nothing has been reported yet. Command outcomes from every tool collect here.
          </Typography>
        )}
      </Box>
    </Box>
  );
}
