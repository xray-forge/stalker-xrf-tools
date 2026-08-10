import { jest } from "@jest/globals";

export type InvokeHandler = (args?: Record<string, unknown>) => unknown;

/**
 * Responses keyed by tauri command name.
 *
 * Anything not listed resolves to `null` rather than throwing, so a test only has to describe the
 * commands it actually cares about.
 */
export type InvokeMap = Record<string, unknown | InvokeHandler>;

const state: { handlers: InvokeMap } = { handlers: {} };

/**
 * Point the mocked `invoke` at a new set of responses.
 *
 * The browser preview cannot run these editors at all, because `invoke` does not exist outside the
 * tauri runtime and every service fails its provisioning. Driving it from here is what makes those
 * workspaces testable without building the rust application.
 */
export function setMockInvokeResponses(handlers: InvokeMap): void {
  state.handlers = handlers;
}

export function resetMockInvoke(): void {
  state.handlers = {};
}

export const mockInvoke = jest.fn(async (command: string, args?: Record<string, unknown>): Promise<unknown> => {
  const handler: unknown = state.handlers[command];

  if (typeof handler === "function") {
    return (handler as InvokeHandler)(args);
  }

  return handler ?? null;
});
