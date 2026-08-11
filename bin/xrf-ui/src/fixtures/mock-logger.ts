import { Logger } from "@/lib/logging";

/** Disable application logging for tests. */
export function mockLogger(): void {
  Logger.IS_GLOBAL_LOGGING_ENABLED = false;
}
