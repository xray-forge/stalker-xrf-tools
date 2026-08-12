/** How urgent a recorded outcome is. Ordered by `NOTIFICATION_SEVERITY_RANK`, not by declaration. */
export const enum ENotificationSeverity {
  SUCCESS = "success",
  INFO = "info",
  WARNING = "warning",
  ERROR = "error",
}

/**
 * Rank used to colour the unread badge when records of several severities are waiting.
 *
 * The badge shows one colour for a mixed pile, so it has to be the most urgent one - a failure hidden
 * behind four successes is exactly the case the panel exists for.
 */
export const NOTIFICATION_SEVERITY_RANK: Record<ENotificationSeverity, number> = {
  [ENotificationSeverity.SUCCESS]: 0,
  [ENotificationSeverity.INFO]: 1,
  [ENotificationSeverity.WARNING]: 2,
  [ENotificationSeverity.ERROR]: 3,
};

/**
 * What an emitter supplies. Everything else about a record is stamped by the service.
 */
export interface INotificationPayload {
  severity: ENotificationSeverity;
  /** Tool the outcome belongs to, so the panel names it the way the rail does. */
  source: string;
  title: string;
  /** Longer context - a path, an error message, a count - revealed when the row is expanded. */
  details?: string;
}

export interface INotification extends INotificationPayload {
  id: string;
  createdAt: number;
  isRead: boolean;
}
