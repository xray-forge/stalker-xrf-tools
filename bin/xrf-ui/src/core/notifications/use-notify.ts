import { EventBus } from "@wirestate/core";
import { useInjection } from "@wirestate/react";
import { useCallback } from "react";

import { emitNotification } from "@/core/notifications/event";
import { INotificationPayload } from "@/core/notifications/types";

export type TNotify = (payload: INotificationPayload) => void;

/**
 * Raise notifications from a component.
 */
export function useNotify(): TNotify {
  const eventBus: EventBus = useInjection(EventBus);

  return useCallback((payload: INotificationPayload) => emitNotification(eventBus, payload), [eventBus]);
}
