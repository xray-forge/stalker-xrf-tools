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

/** Configure responses for mocked Tauri commands. */
export function setMockInvokeResponses(handlers: InvokeMap): void {
  state.handlers = handlers;
}

/** Clear all mocked Tauri command responses. */
export function resetMockInvoke(): void {
  state.handlers = {};
}

const windowState: { isMaximized: boolean; listeners: Array<() => void> } = { isMaximized: false, listeners: [] };

/** Provide the mocked Tauri window used by component tests. */
export const mockAppWindow = {
  isMaximized: jest.fn(async (): Promise<boolean> => windowState.isMaximized),
  minimize: jest.fn(async (): Promise<void> => undefined),
  close: jest.fn(async (): Promise<void> => undefined),
  toggleMaximize: jest.fn(async (): Promise<void> => setMockWindowMaximized(!windowState.isMaximized)),
  onResized: jest.fn(async (handler: () => void): Promise<() => void> => {
    windowState.listeners.push(handler);

    return () => {
      windowState.listeners = windowState.listeners.filter((it: () => void) => it !== handler);
    };
  }),
};

/** Set the mocked window maximized state and notify listeners. */
export function setMockWindowMaximized(next: boolean): void {
  windowState.isMaximized = next;

  for (const listener of windowState.listeners) {
    listener();
  }
}

/** Restore the mocked window to its initial state. */
export function resetMockAppWindow(): void {
  windowState.isMaximized = false;
  windowState.listeners = [];
}

/** Invoke a configured mocked Tauri command. */
export const mockInvoke = jest.fn(async (command: string, args?: Record<string, unknown>): Promise<unknown> => {
  const handler: unknown = state.handlers[command];

  if (typeof handler === "function") {
    return (handler as InvokeHandler)(args);
  }

  return handler ?? null;
});
