import {
  Vector3d,
  VisualBounds,
  VisualDescription,
  VisualSection,
  VisualSubmesh,
} from "@/core/bindings/xrf-visual";
import { Nullable } from "@/lib/types/general";

/** Framing values a camera needs, derived from what the model actually spans. */
export interface IVisualCameraFit {
  center: [number, number, number];
  radius: number;
}

/** One submesh's attributes as views over the shared buffer, plus the range that draws it. */
export interface IVisualSubmeshViews {
  index: number;
  label: string;
  positions: Float32Array;
  normals: Float32Array;
  uvs: Float32Array;
  indices: Uint16Array;
  drawStart: number;
  drawCount: number;
  triangleCount: number;
}

/** Everything the scene needs to build meshes, and nothing it does not. */
export interface IVisualModelViews {
  submeshes: Array<IVisualSubmeshViews>;
  fit: IVisualCameraFit;
  vertexCount: number;
  triangleCount: number;
}

/**
 * Radius used when a model reports no usable extent, so a camera still has somewhere to stand.
 */
const FALLBACK_FIT_RADIUS: number = 1;

/**
 * Reads one coordinate triple, or null when any component is absent.
 *
 * Rust `f32` crosses as `number | null` because a non-finite float serialises to null, and such values
 * do occur: two visuals in the reference trees declare bounds of `f32::MAX`. Treating null as zero would
 * quietly place a model at the origin, so it is treated as no value at all.
 */
function toFiniteTriple(vector: Vector3d): Nullable<[number, number, number]> {
  const { x, y, z } = vector;

  if (x === null || y === null || z === null) {
    return null;
  }

  return Number.isFinite(x) && Number.isFinite(y) && Number.isFinite(z) ? [x, y, z] : null;
}

/**
 * Builds a typed array view over one packed section.
 *
 * Views rather than copies: the whole point of transferring one buffer is that the attributes are used
 * where they landed. Byte offsets are aligned by the packer, which is what makes these constructors
 * legal at all.
 */
function toFloatView(buffer: ArrayBuffer, section: VisualSection): Float32Array {
  return new Float32Array(buffer, section.byteOffset, section.byteLength / Float32Array.BYTES_PER_ELEMENT);
}

function toIndexView(buffer: ArrayBuffer, section: VisualSection): Uint16Array {
  return new Uint16Array(buffer, section.byteOffset, section.byteLength / Uint16Array.BYTES_PER_ELEMENT);
}

/**
 * Framing for a model, preferring what its geometry spans over what its header claims.
 *
 * Measured bounds are the honest ones. Declared bounds are the fallback for a model that produced no
 * geometry, so an empty viewport still frames where the model says it is.
 */
export function createVisualCameraFit(description: VisualDescription): IVisualCameraFit {
  const bounds: Nullable<VisualBounds> = description.computedBounds ?? description.declaredBounds ?? null;
  const center: Nullable<[number, number, number]> = bounds ? toFiniteTriple(bounds.boundingSphere.center) : null;
  const radius: Nullable<number> = bounds?.boundingSphere.radius ?? null;

  return {
    center: center ?? [0, 0, 0],
    radius: radius !== null && Number.isFinite(radius) && radius > 0 ? radius : FALLBACK_FIT_RADIUS,
  };
}

/**
 * Turn a description and its buffer into the views a scene uploads.
 *
 * Deliberately pure and free of three.js, because the offset arithmetic here is the riskiest code in the
 * viewer and the only kind of mistake that renders as a plausible but wrong mesh rather than as an
 * error. Keeping it a function means it is tested without a gpu.
 *
 * @param description - What the backend said the buffer contains.
 * @param buffer - The packed attribute bytes.
 * @returns Per submesh views, draw ranges and camera framing.
 */
export function createVisualViews(description: VisualDescription, buffer: ArrayBuffer): IVisualModelViews {
  if (buffer.byteLength !== description.bufferLength) {
    throw new Error(
      `Geometry buffer is ${buffer.byteLength} bytes but its description covers ${description.bufferLength}. ` +
        "The description and the buffer came from different reads."
    );
  }

  const submeshes: Array<IVisualSubmeshViews> = [];

  let vertexCount: number = 0;
  let triangleCount: number = 0;

  for (const submesh of description.submeshes as Array<VisualSubmesh>) {
    if (submesh.content.kind !== "packed") {
      continue;
    }

    const { geometry } = submesh.content;
    const drawCount: number = geometry.drawRange.count;

    vertexCount += geometry.vertexCount;
    triangleCount += drawCount / 3;

    submeshes.push({
      index: submesh.index,
      label: submesh.textureName ?? `submesh ${submesh.index}`,
      positions: toFloatView(buffer, geometry.positions),
      normals: toFloatView(buffer, geometry.normals),
      uvs: toFloatView(buffer, geometry.uvs),
      indices: toIndexView(buffer, geometry.indices),
      drawStart: geometry.drawRange.start,
      drawCount,
      triangleCount: drawCount / 3,
    });
  }

  return {
    submeshes,
    fit: createVisualCameraFit(description),
    vertexCount,
    triangleCount,
  };
}
