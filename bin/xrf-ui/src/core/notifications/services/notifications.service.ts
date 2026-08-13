import { Injectable, OnEvent, WireEvent } from "@wirestate/core";
import { BoundAction, Computed, makeObservable, Observable } from "@wirestate/mobx";

import {
  ENotificationSeverity,
  INotification,
  INotificationPayload,
  NOTIFICATION_PUSH_EVENT,
  NOTIFICATION_SEVERITY_RANK,
} from "@/core/notifications";
import { Logger } from "@/lib/logging";
import { Nullable } from "@/lib/types/general";

/**
 * The application wide record of what commands did.
 */
@Injectable()
export class NotificationsService {
  /** Ring cap. A loop that reports per file would otherwise grow the panel until it stops being read. */
  public static readonly LIMIT: number = 200;

  /** Dev traces get their own budget, so a chatty one cannot evict the failure being chased. */
  public static readonly DEV_LIMIT: number = 100;

  public readonly log: Logger = new Logger(this.constructor.name);

  private nextId: number = 0;

  /** Newest first, so the panel renders the array as it stands and the cap drops the tail. */
  @Observable()
  public notifications: Array<INotification> = [];

  /** Dev traces, recorded whatever the dev mode switch says. */
  @Observable()
  public devNotifications: Array<INotification> = [];

  /**
   * Both lists in one chronology, which is the reading dev mode is turned on for.
   *
   * Computed rather than a plain getter: it allocates and sorts, and a fresh array on every read is
   * one nobody can compare by reference.
   */
  @Computed()
  public get allNotifications(): Array<INotification> {
    // Ids are a monotonic counter, so they order records that share a millisecond.
    return [...this.notifications, ...this.devNotifications].sort(
      (first: INotification, second: INotification) => Number(second.id) - Number(first.id)
    );
  }

  /** What the badge counts. Dev traces are not in this list, so they cannot light it. */
  @Computed()
  public get unreadCount(): number {
    return this.notifications.reduce((count: number, it: INotification) => (it.isRead ? count : count + 1), 0);
  }

  /** Severity the badge takes its colour from, or null when nothing is unread. */
  @Computed()
  public get highestUnreadSeverity(): Nullable<ENotificationSeverity> {
    let highest: Nullable<ENotificationSeverity> = null;

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

    if (payload.severity === ENotificationSeverity.DEV) {
      this.devNotifications = [notification, ...this.devNotifications].slice(0, NotificationsService.DEV_LIMIT);

      return;
    }

    this.log.info("Notification pushed:", payload.title, notification);

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
      this.log.info("Marking all notifications as read");
      this.notifications = this.notifications.map((it: INotification) => (it.isRead ? it : { ...it, isRead: true }));
    }
  }

  @BoundAction()
  public clear(): void {
    this.log.info("Clear all notifications");
    this.notifications = [];
    this.devNotifications = [];
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
