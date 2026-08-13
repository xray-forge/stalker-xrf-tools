import { IEditorPanel } from "@/core/shell/panel/context";
import { NotificationsPanel } from "@/core/shell/panel/notifications/NotificationsPanel";
import { NotificationsPanelIcon } from "@/core/shell/panel/notifications/NotificationsPanelIcon";

/** The notification log owned by the frame and available in every application. */
export const NOTIFICATIONS_PANEL: IEditorPanel = {
  icon: <NotificationsPanelIcon />,
  id: "notifications",
  isOpenByDefault: false,
  label: "Notifications",
  render: () => <NotificationsPanel />,
  side: "right",
};
