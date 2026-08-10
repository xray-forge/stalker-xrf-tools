const path = require("node:path");

const ROOT_DIR = path.resolve(__dirname, "../../");

/**
 * Jest configuration for the desktop frontend.
 *
 * The config is `.cjs` rather than `.ts` because this package has no `ts-node`; adding one purely to
 * parse a config would be its own cost. Test authors see the same `@jest/globals` API either way.
 *
 * @type {import('jest').Config}
 */
module.exports = {
  cacheDirectory: "<rootDir>/target/jest_cache",
  clearMocks: true,
  coverageDirectory: "<rootDir>/target/coverage_report",
  coveragePathIgnorePatterns: ["/node_modules/", "<rootDir>/src/fixtures/"],
  moduleNameMapper: {
    "\\.(css|less|svg|png|jpg|woff2?)$": path.resolve(__dirname, "./asset-stub.cjs"),
    "^@/(.*)$": "<rootDir>/src/$1",
  },
  rootDir: ROOT_DIR,
  roots: ["<rootDir>"],
  setupFiles: [path.resolve(__dirname, "./jest-global.cjs")],
  setupFilesAfterEnv: [path.resolve(__dirname, "./jest-setup.ts")],
  testEnvironment: "jsdom",
  testMatch: ["<rootDir>/src/**/*.test.{ts,tsx}"],
  transform: {
    "^.+\\.[jt]sx?$": path.resolve(__dirname, "./transformer.cjs"),
  },
  // Every dependency here ships a CJS build; re-compiling them detaches runtimes from their peers.
  transformIgnorePatterns: ["/node_modules/"],
  verbose: true,
  workerIdleMemoryLimit: "512MB",
};
