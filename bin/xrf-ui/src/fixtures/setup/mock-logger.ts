import { jest } from "@jest/globals";

import { noop } from "@/lib/callbacks";
import { Logger } from "@/lib/logging";

/**
 * Disable application logging for tests.
 */
export function mockLogger(): void {
  Logger.IS_GLOBAL_LOGGING_ENABLED = false;

  // The static logger binds console methods while the module loads, before test setup can flip the flag.
  jest.spyOn(Logger, "log").mockImplementation(noop);
  jest.spyOn(Logger, "info").mockImplementation(noop);
  jest.spyOn(Logger, "warn").mockImplementation(noop);
  jest.spyOn(Logger, "error").mockImplementation(noop);
  jest.spyOn(Logger, "debug").mockImplementation(noop);
}
