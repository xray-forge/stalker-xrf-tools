import { IApplicationDescriptor } from "@/core/router/application";
import { APPLICATIONS } from "@/core/router/applications";
import { Maybe } from "@/core/types/general";
import { Logger, useLogger } from "@/lib/logging";
import { useMountEffect } from "@/lib/react";

const PRELOAD_FALLBACK_DELAY: number = 1_000;

type TIdleScheduler = (callback: () => void) => number;

/**
 * Schedule warming for after the window is usable.
 *
 * `requestIdleCallback` where the webview has it, a timeout otherwise - Safari-based webviews and jsdom
 * do not, and this must not be the reason a platform fails to start.
 */
function scheduleWhenIdle(callback: () => void): void {
  const scheduler: Maybe<TIdleScheduler> = (window as unknown as { requestIdleCallback?: TIdleScheduler })
    .requestIdleCallback;

  if (scheduler) {
    scheduler(callback);
  } else {
    window.setTimeout(callback, PRELOAD_FALLBACK_DELAY);
  }
}

/**
 * Which applications the idle warmer pulls in.
 */
export function selectPreloadedApplications(
  applications: Array<IApplicationDescriptor>
): Array<IApplicationDescriptor> {
  return applications.filter((it: IApplicationDescriptor) => it.preload);
}

/**
 * Pull the split application chunks in once the shell is up.
 */
export function useApplicationPreload(): void {
  const log: Logger = useLogger("application-preload");

  useMountEffect(() => {
    const warmed: Array<IApplicationDescriptor> = APPLICATIONS.filter((it: IApplicationDescriptor) => it.preload);

    if (!warmed.length) {
      return;
    }

    scheduleWhenIdle(() => {
      log.debug("Warming application chunks:", warmed.map((it: IApplicationDescriptor) => it.id).join(", "));

      for (const application of warmed) {
        application
          .preload?.()
          .then(() => log.debug("Preloaded:", application.id))
          .catch((error: unknown) => {
            log.error("Failed to warm application chunk:", application.id, error);
          });
      }
    });
  });
}
