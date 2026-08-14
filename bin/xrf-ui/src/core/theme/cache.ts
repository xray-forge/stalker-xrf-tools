import { default as createCache, EmotionCache } from "@emotion/cache";

/**
 * The style cache every MUI component injects through.
 *
 * @returns The application's Emotion style cache.
 */
export function createApplicationStyleCache(): EmotionCache {
  return createCache({ key: "css", speedy: true });
}
