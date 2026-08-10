import "@testing-library/jest-dom";

import { afterEach } from "@jest/globals";
import { cleanup } from "@testing-library/react";

import { mockLogger } from "@/fixtures/mock-logger";
import { mockTauri } from "@/fixtures/mock-tauri";
import { resetMockInvoke } from "@/fixtures/tauri.mocks";

mockLogger();
mockTauri();

afterEach(() => {
  cleanup();
  resetMockInvoke();
});
