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

const windowState: { isMaximized: boolean; listeners: Array<() => void> } = { isMaximized: false, listeners: [] };

/**
 * Stand in for the handle `getCurrentWindow` hands back inside a tauri webview.
 *
 * Only the caption's surface is modelled. `onResized` keeps its listeners so a test can assert that the
 * bar follows the window when it is maximized by something other than its own buttons.
 */
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

/** Maximize or restore the fake window the way the system would, listeners included. */
export function setMockWindowMaximized(next: boolean): void {
  windowState.isMaximized = next;

  for (const listener of windowState.listeners) {
    listener();
  }
}

export function resetMockAppWindow(): void {
  windowState.isMaximized = false;
  windowState.listeners = [];
}

export const mockInvoke = jest.fn(async (command: string, args?: Record<string, unknown>): Promise<unknown> => {
  const handler: unknown = state.handlers[command];

  if (typeof handler === "function") {
    return (handler as InvokeHandler)(args);
  }

  return handler ?? null;
});
