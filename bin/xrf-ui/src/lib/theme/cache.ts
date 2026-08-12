import { default as createCache, EmotionCache } from "@emotion/cache";

/**
 * The style cache every MUI component injects through.
 */
export function createApplicationStyleCache(): EmotionCache {
  return createCache({ key: "css", speedy: true });
}
