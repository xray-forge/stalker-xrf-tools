import { EventBus, inject, Injectable, OnDeactivation, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable } from "@wirestate/mobx";

import { transformError } from "@/core/error";
import { emitNotification, ENotificationSeverity } from "@/core/notifications";
import { APPLICATION_SOURCE } from "@/core/routing/application";
import { Logger } from "@/lib/logging";

/**
 * Records the failures nothing else reports.
 */
@Injectable()
export class ErrorCaptureService {
  public readonly log: Logger = new Logger(this.constructor.name);

  /** Set while a capture is being recorded, so a failure in that path cannot re-enter and loop. */
  private isRecording: boolean = false;

  public constructor(private readonly eventBus: EventBus = inject(EventBus)) {
    makeObservable(this);
  }

  @OnProvision()
  public onProvision(): void {
    window.addEventListener("error", this.onWindowError);
    window.addEventListener("unhandledrejection", this.onUnhandledRejection);
  }

  @OnDeactivation()
  public onDeactivation(): void {
    window.removeEventListener("error", this.onWindowError);
    window.removeEventListener("unhandledrejection", this.onUnhandledRejection);
  }

  @BoundAction()
  public onWindowError(event: ErrorEvent): void {
    // Resource load failures reach the same event without an `error`, and say nothing worth recording.
    if (!event.error && !event.message) {
      return;
    }

    const where: string = event.filename ? `${event.filename}:${event.lineno}:${event.colno}` : "unknown location";

    this.record(event.error ? transformError(event.error).message : event.message, where);
  }

  @BoundAction()
  public onUnhandledRejection(event: PromiseRejectionEvent): void {
    this.record(transformError(event.reason).message, "unhandled rejection");
  }

  /**
   * Record one captured failure, guarding against the loop where recording it fails again.
   */
  private record(message: string, where: string): void {
    if (this.isRecording) {
      return;
    }

    this.isRecording = true;

    try {
      emitNotification(this.eventBus, {
        details: where,
        severity: ENotificationSeverity.DEV,
        source: APPLICATION_SOURCE,
        title: message,
      });
    } finally {
      this.isRecording = false;
    }
  }
}
