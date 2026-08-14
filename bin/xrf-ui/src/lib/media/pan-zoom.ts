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

/**
 * Clamps a scale to the supported viewport range.
 *
 * @param scale - Desired scale, where 1 is one content pixel per viewport pixel.
 * @returns The scale, bounded by the minimum and maximum this module allows.
 */
export function clampScale(scale: number): number {
  return Math.min(PAN_ZOOM_MAXIMUM_SCALE, Math.max(PAN_ZOOM_MINIMUM_SCALE, scale));
}

/**
 * Rescales while keeping the content under `point` fixed.
 *
 * Zooming about the viewport centre instead makes the thing being inspected drift off screen at high
 * magnification, which is when it matters most. The anchor point is in viewport coordinates and the
 * transform it produces is `translate(offset) scale(scale)` with an origin of `0 0`.
 *
 * @param state - Current pan and zoom.
 * @param point - Anchor to hold still, in viewport coordinates.
 * @param nextScale - Desired scale, clamped before use.
 * @returns The pan and zoom placing the same content under the anchor at the new scale.
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

/**
 * Applies one wheel zoom step at the pointer.
 *
 * @param state - Current pan and zoom.
 * @param point - Pointer position to zoom about, in viewport coordinates.
 * @param delta - Wheel delta, where positive scrolls down and so zooms out.
 * @returns The pan and zoom one notch away, anchored on the pointer.
 */
export function zoomByWheel(state: IPanZoomState, point: IPanZoomPoint, delta: number): IPanZoomState {
  return zoomAround(state, point, delta > 0 ? state.scale / WHEEL_STEP : state.scale * WHEEL_STEP);
}

/**
 * Shifts the content without changing its scale.
 *
 * @param state - Current pan and zoom.
 * @param deltaX - Horizontal movement in viewport pixels.
 * @param deltaY - Vertical movement in viewport pixels.
 * @returns The pan and zoom offset by the movement.
 */
export function panBy(state: IPanZoomState, deltaX: number, deltaY: number): IPanZoomState {
  return { ...state, offsetX: state.offsetX + deltaX, offsetY: state.offsetY + deltaY };
}

/**
 * Scales content to fit within the viewport and centres it.
 *
 * Never enlarges: something smaller than the viewport is shown at its own size, because blowing a 16px
 * icon up to fill a pane tells you less about it than seeing how small it really is.
 *
 * @param content - Content size in its own pixels.
 * @param viewport - Viewport size in viewport pixels.
 * @returns The centred pan and zoom, or the identity when either size is not yet known.
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
