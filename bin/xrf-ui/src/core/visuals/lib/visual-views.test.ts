import { describe, expect, it } from "@jest/globals";

import { VisualDescription } from "@/core/bindings/types/xrf-visual";
import { createVisualCameraFit, createVisualViews, IVisualModelViews } from "@/core/visuals/lib/visual-views";
import {
  mockPackedSubmesh,
  mockSkippedSubmesh,
  mockVisualBounds,
  MockVisualBuffer,
  mockVisualDescription,
} from "@/fixtures/mocks/visual.mocks";

describe("visual views", () => {
  it("builds typed array views over the packed sections", () => {
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const description: VisualDescription = mockVisualDescription({
      submeshes: [mockPackedSubmesh(buffer)],
      bufferLength: buffer.byteLength,
    });

    const views: IVisualModelViews = createVisualViews(description, buffer.toArrayBuffer());

    expect(views.submeshes).toHaveLength(1);
    expect(Array.from(views.submeshes[0].positions)).toEqual([0, 0, 0, 1, 0, 0, 0, 1, 0]);
    expect(Array.from(views.submeshes[0].uvs)).toEqual([0, 0, 1, 0, 0, 1]);
    expect(Array.from(views.submeshes[0].indices)).toEqual([0, 1, 2]);
  });

  it("rejects a buffer whose length disagrees with its description", () => {
    // The pair is fetched in two calls, so a mismatch means they came from different reads. Building
    // views anyway would render whatever the offsets happened to land on.
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const description: VisualDescription = mockVisualDescription({
      submeshes: [mockPackedSubmesh(buffer)],
      bufferLength: buffer.byteLength + 4,
    });

    expect(() => createVisualViews(description, buffer.toArrayBuffer())).toThrow(/came from different reads/);
  });

  it("keeps the draw range separate from the whole index buffer", () => {
    // A progressive submesh ships every detail level, and only the first is drawn. Uploading the buffer
    // but drawing the reported range is the whole arrangement.
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const submesh = mockPackedSubmesh(
      buffer,
      {},
      { indexCount: 12, drawRange: { start: 6, count: 6 }, windows: [{ offset: 6, triangleCount: 2, vertexCount: 6 }] }
    );
    const description: VisualDescription = mockVisualDescription({
      submeshes: [submesh],
      bufferLength: buffer.byteLength,
    });

    const views: IVisualModelViews = createVisualViews(description, buffer.toArrayBuffer());

    expect(views.submeshes[0].drawStart).toBe(6);
    expect(views.submeshes[0].drawCount).toBe(6);
    expect(views.submeshes[0].triangleCount).toBe(2);
    expect(views.triangleCount).toBe(2);
  });

  it("leaves out submeshes that packed nothing", () => {
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const packed = mockPackedSubmesh(buffer, { index: 0 });
    const description: VisualDescription = mockVisualDescription({
      submeshes: [packed, mockSkippedSubmesh({ index: 1 })],
      bufferLength: buffer.byteLength,
    });

    const views: IVisualModelViews = createVisualViews(description, buffer.toArrayBuffer());

    expect(views.submeshes.map((it) => it.index)).toEqual([0]);
    expect(views.vertexCount).toBe(3);
  });

  it("labels a submesh by its texture, falling back to its index", () => {
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const description: VisualDescription = mockVisualDescription({
      submeshes: [
        mockPackedSubmesh(buffer, { index: 0, textureName: "wpn\\wpn_ak74" }),
        mockPackedSubmesh(buffer, { index: 1, textureName: null }),
      ],
      bufferLength: buffer.byteLength,
    });

    const views: IVisualModelViews = createVisualViews(description, buffer.toArrayBuffer());

    expect(views.submeshes.map((it) => it.label)).toEqual(["wpn\\wpn_ak74", "submesh 1"]);
  });

  it("packs several submeshes into one buffer without overlapping", () => {
    const buffer: MockVisualBuffer = new MockVisualBuffer();
    const description: VisualDescription = mockVisualDescription({
      submeshes: [mockPackedSubmesh(buffer, { index: 0 }), mockPackedSubmesh(buffer, { index: 1 })],
      bufferLength: buffer.byteLength,
    });

    const views: IVisualModelViews = createVisualViews(description, buffer.toArrayBuffer());

    expect(views.submeshes).toHaveLength(2);
    expect(views.submeshes[1].positions.byteOffset).toBeGreaterThan(views.submeshes[0].indices.byteOffset);
    expect(views.vertexCount).toBe(6);
  });
});

describe("visual camera fit", () => {
  it("frames what the geometry spans rather than what the header claims", () => {
    const fit = createVisualCameraFit(
      mockVisualDescription({
        declaredBounds: mockVisualBounds({ boundingSphere: { center: { x: 9, y: 9, z: 9 }, radius: 99 } }),
        computedBounds: mockVisualBounds({ boundingSphere: { center: { x: 1, y: 2, z: 3 }, radius: 4 } }),
      })
    );

    expect(fit).toEqual({ center: [1, 2, 3], radius: 4 });
  });

  it("falls back to the declared extent when nothing packed", () => {
    const fit = createVisualCameraFit(
      mockVisualDescription({
        declaredBounds: mockVisualBounds({ boundingSphere: { center: { x: 5, y: 0, z: 0 }, radius: 7 } }),
        computedBounds: null,
      })
    );

    expect(fit).toEqual({ center: [5, 0, 0], radius: 7 });
  });

  it("treats an absent coordinate as no value rather than as zero", () => {
    // A rust f32 crosses as `number | null`, and two visuals in the reference trees declare bounds of
    // f32::MAX. Reading null as zero would place the camera at the origin and claim it framed the model.
    const fit = createVisualCameraFit(
      mockVisualDescription({
        computedBounds: mockVisualBounds({ boundingSphere: { center: { x: null, y: 2, z: 3 }, radius: null } }),
      })
    );

    expect(fit.center).toEqual([0, 0, 0]);
    expect(fit.radius).toBe(1);
    expect(Number.isFinite(fit.radius)).toBe(true);
  });

  it("refuses a degenerate radius so a camera still has somewhere to stand", () => {
    const fit = createVisualCameraFit(
      mockVisualDescription({
        computedBounds: mockVisualBounds({ boundingSphere: { center: { x: 0, y: 0, z: 0 }, radius: 0 } }),
      })
    );

    expect(fit.radius).toBe(1);
  });
});
