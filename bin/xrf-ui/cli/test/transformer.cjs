const crypto = require("node:crypto");
const fs = require("node:fs");

const babel = require("@babel/core");
const swcJest = require("@swc/jest");
const observingComponents = require("babel-plugin-observing-components");

// The package default-exports a factory returning a `[plugin, options]` tuple, so it has to be called
// rather than referenced by name. This is the identical call `mobx-react-observer/vite-plugin` makes.
const createObserverPlugin = observingComponents.default ?? observingComponents;

const SELF_HASH = crypto.createHash("sha1").update(fs.readFileSync(__filename)).digest("hex").slice(0, 12);

/**
 * Compile with swc, after wrapping components in `observer()` with babel.
 *
 * Two transforms in the order `vite.config.ts` applies them: the observer plugin runs `pre` on source
 * that still has its JSX, then the typescript compile happens. Splitting the work this way is not a
 * preference - the observer plugin exists only for babel, while decorators need swc, which implements
 * typescript's `experimentalDecorators` semantics rather than babel's approximation of them. Babel's
 * legacy decorator transform produced services whose `@Observable()` fields were not reactive.
 */
function createSwcOptions(isTsx) {
  return {
    jsc: {
      parser: {
        syntax: "typescript",
        tsx: isTsx,
        decorators: true,
      },
      target: "es2022",
      transform: {
        decoratorMetadata: true,
        legacyDecorator: true,
        react: {
          development: true,
          runtime: "automatic",
        },
      },
    },
    module: { type: "commonjs" },
    sourceMaps: true,
  };
}

const tsxTransformer = swcJest.createTransformer(createSwcOptions(true));
const tsTransformer = swcJest.createTransformer(createSwcOptions(false));

function applyObserver(source, filename) {
  // Mirrors the `.tsx` guard in `vite.config.ts`: only component files carry JSX worth wrapping.
  if (!filename.endsWith(".tsx") || filename.includes("node_modules")) {
    return source;
  }

  const result = babel.transformSync(source, {
    babelrc: false,
    configFile: false,
    filename,
    parserOpts: { plugins: ["jsx", "typescript"] },
    plugins: [createObserverPlugin({ importPath: "mobx-react-observer" })],
  });

  return result?.code ?? source;
}

module.exports = {
  process(source, filename, options) {
    const transformer = filename.endsWith(".tsx") ? tsxTransformer : tsTransformer;

    return transformer.process(applyObserver(source, filename), filename, options);
  },
  getCacheKey(source, filename, options) {
    const transformer = filename.endsWith(".tsx") ? tsxTransformer : tsTransformer;

    // Salted with this file's own contents. swc's key covers the source and its own options but knows
    // nothing about the observer step, so without this an edit here silently reuses output compiled
    // by the previous version - which is exactly how a broken transform passes its own tests.
    return `${transformer.getCacheKey(source, filename, options)}-${SELF_HASH}`;
  },
};
