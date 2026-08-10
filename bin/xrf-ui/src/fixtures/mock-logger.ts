import { Logger } from "@/lib/logging";

/**
 * Silence application logging for the duration of the test run.
 * A test that wants to assert on logging can flip the flag back on for its own scope.
 */
export function mockLogger(): void {
  Logger.IS_GLOBAL_LOGGING_ENABLED = false;
}
