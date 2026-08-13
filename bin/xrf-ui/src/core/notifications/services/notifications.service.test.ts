import { describe, expect, it } from "@jest/globals";
import { Container, EventBus, EventsPlugin } from "@wirestate/core";

import { emitNotification, ENotificationSeverity, INotification } from "@/core/notifications";
import { NotificationsService } from "@/core/notifications/services/notifications.service";
import { EApplicationId } from "@/core/routing/application";

function createService(): NotificationsService {
  return new Container({ bindings: [NotificationsService] }).get(NotificationsService);
}

describe("NotificationsService", () => {
  it("stamps a record and keeps the newest first", () => {
    const service: NotificationsService = createService();

    service.push({ severity: ENotificationSeverity.SUCCESS, source: EApplicationId.ARCHIVES, title: "First" });
    service.push({ severity: ENotificationSeverity.ERROR, source: EApplicationId.ARCHIVES, title: "Second" });

    expect(service.notifications.map((it: INotification) => it.title)).toEqual(["Second", "First"]);
    expect(service.notifications.every((it: INotification) => Boolean(it.id) && !it.isRead)).toBe(true);
    expect(new Set(service.notifications.map((it: INotification) => it.id)).size).toBe(2);
  });

  it("drops the oldest record rather than growing without bound", () => {
    const service: NotificationsService = createService();

    for (let it = 0; it <= NotificationsService.LIMIT; it += 1) {
      service.push({
        severity: ENotificationSeverity.INFO,
        source: EApplicationId.ARCHIVES,
        title: `Record ${it}`,
      });
    }

    expect(service.notifications).toHaveLength(NotificationsService.LIMIT);
    expect(service.notifications[0].title).toBe(`Record ${NotificationsService.LIMIT}`);
    expect(service.notifications.at(-1)?.title).toBe("Record 1");
  });

  it("badges the most urgent unread severity, not the newest one", () => {
    const service: NotificationsService = createService();

    service.push({ severity: ENotificationSeverity.ERROR, source: EApplicationId.ARCHIVES, title: "Failed" });
    service.push({ severity: ENotificationSeverity.SUCCESS, source: EApplicationId.ARCHIVES, title: "Worked" });

    expect(service.unreadCount).toBe(2);
    expect(service.highestUnreadSeverity).toBe(ENotificationSeverity.ERROR);
  });

  it("has nothing to badge once everything is read", () => {
    const service: NotificationsService = createService();

    service.push({ severity: ENotificationSeverity.ERROR, source: EApplicationId.ARCHIVES, title: "Failed" });
    service.markAllRead();

    expect(service.unreadCount).toBe(0);
    expect(service.highestUnreadSeverity).toBeNull();
    // Read, not removed - the panel is the record of what happened, and the badge is only the alert.
    expect(service.notifications).toHaveLength(1);
  });

  it("clears everything on request", () => {
    const service: NotificationsService = createService();

    service.push({ severity: ENotificationSeverity.INFO, source: EApplicationId.ARCHIVES, title: "Something" });
    service.clear();

    expect(service.notifications).toHaveLength(0);
  });

  it("records what the event bus delivers, which is how every editor reaches it", () => {
    const container: Container = new Container({
      bindings: [NotificationsService],
      plugins: [new EventsPlugin()],
    });

    container.provision();

    const service: NotificationsService = container.get(NotificationsService);

    emitNotification(container.get(EventBus), {
      details: "C:\\out",
      severity: ENotificationSeverity.SUCCESS,
      source: EApplicationId.ARCHIVES,
      title: "Extracted textures",
    });

    expect(service.notifications).toHaveLength(1);
    expect(service.notifications[0].title).toBe("Extracted textures");
    expect(service.notifications[0].details).toBe("C:\\out");
  });
});
