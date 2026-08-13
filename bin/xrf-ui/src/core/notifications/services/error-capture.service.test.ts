import { beforeEach, describe, expect, it, jest } from "@jest/globals";
import { Container, EventBus, EventsPlugin, WireEvent } from "@wirestate/core";

import { ENotificationSeverity, INotificationPayload, NOTIFICATION_PUSH_EVENT } from "@/core/notifications";
import { ErrorCaptureService } from "@/core/notifications/services/error-capture.service";

interface IWatchedCapture {
  container: Container;
  service: ErrorCaptureService;
  raised: Array<INotificationPayload>;
}

function watchCapture(): IWatchedCapture {
  const container: Container = new Container({
    bindings: [ErrorCaptureService],
    plugins: [new EventsPlugin()],
  });

  const raised: Array<INotificationPayload> = [];

  container
    .get(EventBus)
    .subscribe(NOTIFICATION_PUSH_EVENT, (event: WireEvent<INotificationPayload>) =>
      raised.push(event.payload as INotificationPayload)
    );

  return { container, raised, service: container.get(ErrorCaptureService) };
}

describe("ErrorCaptureService", () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it("registers and releases the same handler references", () => {
    const added: Array<[string, unknown]> = [];
    const removed: Array<[string, unknown]> = [];

    jest.spyOn(window, "addEventListener").mockImplementation((type: string, handler: unknown) => {
      added.push([type, handler]);
    });

    jest.spyOn(window, "removeEventListener").mockImplementation((type: string, handler: unknown) => {
      removed.push([type, handler]);
    });

    const { container }: IWatchedCapture = watchCapture();

    container.provision();
    container.unbindAll();

    // The whole reason the handlers are bound actions rather than plain methods: a fresh function per
    // registration would leave `removeEventListener` a no-op and the listeners attached for good.
    expect(added.map(([type]: [string, unknown]) => type)).toEqual(["error", "unhandledrejection"]);
    expect(removed).toEqual(added);
  });

  it("records a dropped rejection, which nothing else reports", () => {
    const { raised, service }: IWatchedCapture = watchCapture();

    service.onUnhandledRejection({ reason: new Error("nobody caught this") } as PromiseRejectionEvent);

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.DEV);
    expect(raised[0].source).toBe("application");
    expect(raised[0].title).toBe("nobody caught this");
  });

  it("records where an uncaught exception came from", () => {
    const { raised, service }: IWatchedCapture = watchCapture();

    service.onWindowError({
      colno: 7,
      error: new Error("exploded"),
      filename: "http://localhost/src/thing.ts",
      lineno: 42,
      message: "exploded",
    } as ErrorEvent);

    expect(raised).toHaveLength(1);
    expect(raised[0].title).toBe("exploded");
    expect(raised[0].details).toBe("http://localhost/src/thing.ts:42:7");
  });

  it("ignores the resource load failures that arrive on the same event", () => {
    const { raised, service }: IWatchedCapture = watchCapture();

    service.onWindowError({ error: null, message: "" } as unknown as ErrorEvent);

    expect(raised).toHaveLength(0);
  });
});
