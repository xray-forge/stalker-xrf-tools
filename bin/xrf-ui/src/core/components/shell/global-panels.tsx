import { IEditorPanel } from "@/core/components/shell/EditorPanelsContext";
import { NotificationsPanel } from "@/core/components/shell/notifications/NotificationsPanel";
import { NotificationsPanelIcon } from "@/core/components/shell/notifications/NotificationsPanelIcon";

export const NOTIFICATIONS_PANEL_ID: string = "notifications";

/**
 * The notification log, which the frame owns rather than the active application.
 *
 * Pinned to the top of the right stripe rather than listed with the application's panels: it is the
 * mirror of Home, in the same place whatever is open.
 */
export const NOTIFICATIONS_PANEL: IEditorPanel = {
  icon: <NotificationsPanelIcon />,
  id: NOTIFICATIONS_PANEL_ID,
  isOpenByDefault: false,
  label: "Notifications",
  render: () => <NotificationsPanel />,
  side: "right",
};

/** Panels the frame owns. A list so a slot can tell them apart from what an application declared. */
export const GLOBAL_PANELS: Array<IEditorPanel> = [NOTIFICATIONS_PANEL];

export function isGlobalPanelId(id: string): boolean {
  return GLOBAL_PANELS.some((panel: IEditorPanel) => panel.id === id);
}
