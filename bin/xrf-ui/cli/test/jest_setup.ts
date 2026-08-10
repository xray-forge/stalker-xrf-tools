import "@testing-library/jest-dom";

import { afterEach } from "@jest/globals";
import { cleanup } from "@testing-library/react";

import { mockTauri } from "@/fixtures/mock-tauri";
import { resetMockInvoke } from "@/fixtures/tauri.mocks";

mockTauri();

afterEach(() => {
  cleanup();
  resetMockInvoke();
});
