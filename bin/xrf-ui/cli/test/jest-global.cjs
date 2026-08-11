// Gaps in jsdom, filled before any module is imported.
//
// These are not mocks - nothing here stands in for application behaviour. They exist because jsdom
// omits browser and node globals that dependencies reach for at import time, which is earlier than any
// `setupFilesAfterEnv` hook could run. Mocks live in `src/fixtures` and are registered from
// `jest_setup.ts`.
const { TextDecoder, TextEncoder } = require("node:util");

if (typeof global.TextEncoder === "undefined") {
  global.TextEncoder = TextEncoder;
}

if (typeof global.TextDecoder === "undefined") {
  global.TextDecoder = TextDecoder;
}

if (typeof global.structuredClone === "undefined") {
  global.structuredClone = (value) => JSON.parse(JSON.stringify(value));
}

// Used by the preview scene to track its container, and by the shell layout.
if (typeof global.ResizeObserver === "undefined") {
  global.ResizeObserver = class {
    observe() {}
    unobserve() {}
    disconnect() {}
  };
}

// jsdom implements neither, so anything holding an object url is untestable without them. Counting
// rather than no-oping, so a test can assert that what was created was also released.
if (typeof URL.createObjectURL === "undefined") {
  let nextObjectUrlId = 0;

  URL.createObjectURL = () => `blob:mock/${(nextObjectUrlId += 1)}`;
  URL.revokeObjectURL = () => {};
}

// MUI's color scheme support queries this during render.
if (typeof window !== "undefined" && !window.matchMedia) {
  window.matchMedia = (query) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: () => {},
    removeEventListener: () => {},
    addListener: () => {},
    removeListener: () => {},
    dispatchEvent: () => false,
  });
}
