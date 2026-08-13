import { EventBus, EventType } from "@wirestate/core";

import { INotificationPayload } from "@/core/notifications/types";

/**
 * The one event the notification centre listens for.
 */
export const NOTIFICATION_PUSH_EVENT: EventType = Symbol("notification/push");

/**
 * Record an outcome in the notification centre.
 */
export function emitNotification(bus: EventBus, payload: INotificationPayload): void {
  bus.emit<INotificationPayload>(NOTIFICATION_PUSH_EVENT, payload);
}
