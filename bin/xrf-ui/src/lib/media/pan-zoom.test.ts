import { describe, expect, it } from "@jest/globals";

import {
  clampScale,
  fitToViewport,
  IPanZoomState,
  PAN_ZOOM_IDENTITY,
  PAN_ZOOM_MAXIMUM_SCALE,
  PAN_ZOOM_MINIMUM_SCALE,
  panBy,
  zoomAround,
  zoomByWheel,
} from "@/lib/media/pan-zoom";

/**
 * Projects a content-space coordinate into the viewport.
 *
 * @param state - Pan and zoom transform to apply.
 * @param contentX - Horizontal content-space coordinate.
 * @param contentY - Vertical content-space coordinate.
 * @returns The projected viewport coordinate.
 */
function project(state: IPanZoomState, contentX: number, contentY: number): { x: number; y: number } {
  return { x: state.offsetX + contentX * state.scale, y: state.offsetY + contentY * state.scale };
}

describe("pan-zoom", () => {
  it("keeps the anchored point exactly where it was", () => {
    const start: IPanZoomState = { scale: 1, offsetX: 30, offsetY: 10 };
    const anchor = { x: 120, y: 80 };

    // Whatever content sits under the cursor before the zoom has to sit under it afterwards; this is
    // the whole reason for anchoring rather than scaling about the centre.
    const contentX: number = (anchor.x - start.offsetX) / start.scale;
    const contentY: number = (anchor.y - start.offsetY) / start.scale;

    const zoomed: IPanZoomState = zoomAround(start, anchor, 4);
    const after = project(zoomed, contentX, contentY);

    expect(after.x).toBeCloseTo(anchor.x);
    expect(after.y).toBeCloseTo(anchor.y);
    expect(zoomed.scale).toBe(4);
  });

  it("holds the anchor across repeated wheel notches", () => {
    const anchor = { x: 200, y: 140 };

    let state: IPanZoomState = PAN_ZOOM_IDENTITY;

    const contentX: number = anchor.x;
    const contentY: number = anchor.y;

    for (let index = 0; index < 8; index += 1) {
      state = zoomByWheel(state, anchor, -1);
    }

    const after = project(state, contentX, contentY);

    // Drift here would be invisible on one notch and obvious after a few, which is how it would ship.
    expect(after.x).toBeCloseTo(anchor.x);
    expect(after.y).toBeCloseTo(anchor.y);
    expect(state.scale).toBeGreaterThan(1);
  });

  it("zooms out on a downward wheel and in on an upward one", () => {
    expect(zoomByWheel(PAN_ZOOM_IDENTITY, { x: 0, y: 0 }, 1).scale).toBeLessThan(1);
    expect(zoomByWheel(PAN_ZOOM_IDENTITY, { x: 0, y: 0 }, -1).scale).toBeGreaterThan(1);
  });

  it("clamps the scale at both ends", () => {
    expect(clampScale(1000)).toBe(PAN_ZOOM_MAXIMUM_SCALE);
    expect(clampScale(0)).toBe(PAN_ZOOM_MINIMUM_SCALE);
    expect(zoomAround(PAN_ZOOM_IDENTITY, { x: 0, y: 0 }, 1000).scale).toBe(PAN_ZOOM_MAXIMUM_SCALE);
  });

  it("pans by exactly the delta it is given", () => {
    expect(panBy({ scale: 3, offsetX: 10, offsetY: 20 }, -5, 7)).toEqual({ scale: 3, offsetX: 5, offsetY: 27 });
  });

  it("fits a large image and centres it", () => {
    const state: IPanZoomState = fitToViewport({ x: 1024, y: 512 }, { x: 512, y: 512 });

    expect(state.scale).toBe(0.5);
    expect(state.offsetX).toBe(0);
    // Letterboxed vertically, so the remaining space is split evenly.
    expect(state.offsetY).toBe((512 - 512 * 0.5) / 2);
  });

  it("never enlarges something smaller than the viewport", () => {
    const state: IPanZoomState = fitToViewport({ x: 16, y: 16 }, { x: 512, y: 512 });

    // Blowing a 16px icon up to fill the pane says less about it than seeing how small it is.
    expect(state.scale).toBe(1);
  });

  it("falls back to identity when a dimension is unknown", () => {
    expect(fitToViewport({ x: 0, y: 0 }, { x: 512, y: 512 })).toEqual(PAN_ZOOM_IDENTITY);
    expect(fitToViewport({ x: 64, y: 64 }, { x: 0, y: 0 })).toEqual(PAN_ZOOM_IDENTITY);
  });
});
