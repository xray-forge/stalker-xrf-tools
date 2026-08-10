import { jest } from "@jest/globals";

import { mockAppWindow, mockInvoke } from "@/fixtures/tauri.mocks";

/**
 * Mock the tauri surface, none of which exists under jsdom.
 *
 * Registered centrally rather than per test file: a component that quietly reaches for a plugin would
 * otherwise fail in a way that looks like a render bug. Commands are answered through
 * `setMockInvokeResponses`, so a test only describes the commands it cares about.
 *
 * The `jest.mock` calls live inside a function on purpose. They are not hoisted when invoked from a
 * module, so importing this file has no side effect until a setup file calls it - and the factories are
 * free to close over `mockInvoke`, which hoisted mocks could not do.
 */
export function mockTauri(): void {
  jest.mock("@tauri-apps/api/core", () => ({
    invoke: mockInvoke,
    convertFileSrc: (path: string) => `asset://${path}`,
    isTauri: () => true,
  }));

  jest.mock("@tauri-apps/api/window", () => ({
    getCurrentWindow: () => mockAppWindow,
  }));

  jest.mock("@tauri-apps/api", () => ({
    path: {
      join: async (...parts: Array<string>) => parts.join("\\"),
      resolve: async (...parts: Array<string>) => parts.join("\\"),
    },
  }));

  jest.mock("@tauri-apps/plugin-dialog", () => ({
    open: jest.fn(async () => null),
    save: jest.fn(async () => null),
  }));

  jest.mock("@tauri-apps/plugin-fs", () => ({
    exists: jest.fn(async () => true),
  }));

  jest.mock("@tauri-apps/plugin-shell", () => ({
    open: jest.fn(async () => undefined),
  }));
}
