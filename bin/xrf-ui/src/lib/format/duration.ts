/**
 * Render a millisecond duration the way a report should read.
 *
 * The previous result components printed `duration / 1000` straight into the DOM, which produced
 * values like `0.123 sec` for a fast run and `98.4315 sec` for a slow one.
 */
export function formatDuration(durationMs: number): string {
  if (durationMs < 1000) {
    return `${Math.round(durationMs)} ms`;
  }

  const seconds: number = durationMs / 1000;

  if (seconds < 60) {
    return `${seconds.toFixed(1)} s`;
  }

  const minutes: number = Math.floor(seconds / 60);

  return `${minutes} m ${Math.round(seconds - minutes * 60)} s`;
}
