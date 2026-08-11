export interface IPanZoomState {
  scale: number;
  offsetX: number;
  offsetY: number;
}

export interface IPanZoomPoint {
  x: number;
  y: number;
}

export const PAN_ZOOM_MINIMUM_SCALE: number = 0.1;
export const PAN_ZOOM_MAXIMUM_SCALE: number = 32;
export const PAN_ZOOM_IDENTITY: IPanZoomState = { scale: 1, offsetX: 0, offsetY: 0 };

/** One wheel notch, as a multiplier rather than an addend so each step feels the same at any zoom. */
const WHEEL_STEP: number = 1.2;

export function clampScale(scale: number): number {
  return Math.min(PAN_ZOOM_MAXIMUM_SCALE, Math.max(PAN_ZOOM_MINIMUM_SCALE, scale));
}

/**
 * Rescale while keeping whatever sits under `point` exactly where it is.
 *
 * Zooming about the viewport centre instead makes the thing being inspected drift off screen at high
 * magnification, which is when it matters most. The anchor point is in viewport coordinates and the
 * transform it produces is `translate(offset) scale(scale)` with an origin of `0 0`.
 */
export function zoomAround(state: IPanZoomState, point: IPanZoomPoint, nextScale: number): IPanZoomState {
  const scale: number = clampScale(nextScale);

  // The content-space coordinate under the anchor has to map back onto the same viewport coordinate.
  return {
    scale,
    offsetX: point.x - ((point.x - state.offsetX) / state.scale) * scale,
    offsetY: point.y - ((point.y - state.offsetY) / state.scale) * scale,
  };
}

/** Apply one wheel notch at the pointer. Positive `delta` scrolls down, which zooms out. */
export function zoomByWheel(state: IPanZoomState, point: IPanZoomPoint, delta: number): IPanZoomState {
  return zoomAround(state, point, delta > 0 ? state.scale / WHEEL_STEP : state.scale * WHEEL_STEP);
}

export function panBy(state: IPanZoomState, deltaX: number, deltaY: number): IPanZoomState {
  return { ...state, offsetX: state.offsetX + deltaX, offsetY: state.offsetY + deltaY };
}

/**
 * Scale to fit within the viewport and centre it.
 *
 * Never enlarges: something smaller than the viewport is shown at its own size, because blowing a 16px
 * icon up to fill a pane tells you less about it than seeing how small it really is.
 */
export function fitToViewport(content: IPanZoomPoint, viewport: IPanZoomPoint): IPanZoomState {
  if (!content.x || !content.y || !viewport.x || !viewport.y) {
    return PAN_ZOOM_IDENTITY;
  }

  const scale: number = clampScale(Math.min(1, Math.min(viewport.x / content.x, viewport.y / content.y)));

  return {
    scale,
    offsetX: (viewport.x - content.x * scale) / 2,
    offsetY: (viewport.y - content.y * scale) / 2,
  };
}
