import baseConfig from "./eslint.config.mjs";

/**
 * Flat configuration for the generated Rust mirrors, used only by `format:bindings`.
 *
 * Those files are linted by the ordinary configuration like the rest of the sources, and must pass it.
 * This one covers the single moment they are not yet formatted: the fix pass runs first so the interface
 * rewrite happens before prettier decides the wrapping, and until prettier has run, ts-rs output is one
 * line per type with no notion of width. Exempting the rule here rather than in the base configuration
 * keeps the generated output held to the same line length as everything else once it settles.
 */
export default [
  ...baseConfig,
  {
    files: ["src/lib/rust-sdk/**/*.ts"],
    rules: {
      "max-len": "off",
    },
  },
];
