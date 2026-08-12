import { Injectable, OnEvent, WireEvent } from "@wirestate/core";
import { BoundAction, makeObservable, Observable } from "@wirestate/mobx";

import { Nullable } from "@/core/types/general";
import { Logger } from "@/lib/logging";
import {
  INotification,
  INotificationPayload,
  NOTIFICATION_PUSH_EVENT,
  NOTIFICATION_SEVERITY_RANK,
  TNotificationSeverity,
} from "@/lib/notifications";

/**
 * The application wide record of what commands did.
 */
@Injectable()
export class NotificationsService {
  /** Ring cap. A loop that reports per file would otherwise grow the panel until it stops being read. */
  public static readonly LIMIT: number = 200;

  public readonly log: Logger = new Logger(this.constructor.name);

  /** Newest first, so the panel renders the array as it stands and the cap drops the tail. */
  @Observable()
  public notifications: Array<INotification> = [];

  private nextId: number = 0;

  public get unreadCount(): number {
    return this.notifications.reduce((count: number, it: INotification) => (it.isRead ? count : count + 1), 0);
  }

  /** Severity the badge takes its colour from, or null when nothing is unread. */
  public get highestUnreadSeverity(): Nullable<TNotificationSeverity> {
    let highest: Nullable<TNotificationSeverity> = null;

    for (const notification of this.notifications) {
      if (
        !notification.isRead &&
        (highest === null || NOTIFICATION_SEVERITY_RANK[notification.severity] > NOTIFICATION_SEVERITY_RANK[highest])
      ) {
        highest = notification.severity;
      }
    }

    return highest;
  }

  public constructor() {
    makeObservable(this);
  }

  /**
   * Record an outcome raised anywhere in the application.
   *
   * Public as well as bus-driven so a test does not need a provisioned container to describe what the
   * store should do with a record.
   */
  @BoundAction()
  public push(payload: INotificationPayload): void {
    const notification: INotification = {
      ...payload,
      id: String((this.nextId += 1)),
      createdAt: Date.now(),
      isRead: false,
    };

    this.notifications = [notification, ...this.notifications].slice(0, NotificationsService.LIMIT);
  }

  /**
   * Clear the unread badge.
   *
   * Called by the panel while it is open, including for records that arrive while it is open - they
   * were on screen as they landed, so calling them unread would leave a badge nothing can dismiss.
   */
  @BoundAction()
  public markAllRead(): void {
    if (this.unreadCount) {
      this.notifications = this.notifications.map((it: INotification) => (it.isRead ? it : { ...it, isRead: true }));
    }
  }

  @BoundAction()
  public clear(): void {
    this.notifications = [];
  }

  /**
   * Handlers register when the root container is provisioned, which happens before any editor mounts.
   */
  @OnEvent(NOTIFICATION_PUSH_EVENT)
  public onNotificationPush(event: WireEvent<INotificationPayload>): void {
    if (!event.payload) {
      return this.log.warn("Ignoring notification event with no payload");
    }

    this.push(event.payload);
  }
}
