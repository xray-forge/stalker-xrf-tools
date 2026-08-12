/**
 * Whether this bundle was built for development.
 */
export function isDevelopmentBuild(): boolean {
  return process.env.NODE_ENV !== "production";
}
