// Auto-generated rust bindings. Do not edit it manually.

export type Vector3d<T = number | null> = {
  x: T;
  y: T;
  z: T;
};

/**
 * One bone of a visual's skeleton, as a name and the name of its parent.
 *
 * A root bone carries an empty parent. Names rather than indices, because that is how OGF stores the
 * hierarchy and a tree can be rebuilt from them without further work.
 */
export type VisualBone = {
  name: string;
  parent: string;
};

/**
 * A visual's extent, as a box and a sphere.
 *
 * A description carries this twice, unreconciled: once as the values the OGF header declares and
 * once as the values its geometry actually spans. A file whose declared extent disagrees with its
 * vertices then shows the disagreement instead of silently mis-framing a camera.
 *
 * A computed sphere is centred on the computed box and reaches the furthest vertex from that
 * centre. That is an enclosing sphere rather than the minimal one, so a small disagreement with a
 * declared sphere is expected and only a large one is interesting.
 */
export type VisualBounds = {
  boundingBox: VisualBox;
  boundingSphere: VisualSphere;
};

/** Axis aligned box in three.js space. */
export type VisualBox = {
  min: Vector3d;
  max: Vector3d;
};

/**
 * Everything about a packed visual except the bytes themselves.
 *
 * The counterpart of the geometry buffer: a consumer reads this first, then asks for the buffer and
 * builds views from the byte ranges each submesh carries. `buffer_length` is the length that buffer
 * must have, so a mismatched pair is detectable rather than rendering as garbage.
 */
export type VisualDescription = {
  version: number;
  modelType: number;
  modelTypeLabel: string;
  shaderId: number;
  /** Source object the OGF was built from, when the file records one. */
  sourceFile: string | null;
  /** Extent the header declares, converted into three.js space so it compares to `computed_bounds`. */
  declaredBounds: VisualBounds;
  /** Extent the packed geometry actually spans, absent when no submesh produced any. */
  computedBounds: VisualBounds | null;
  submeshes: Array<VisualSubmesh>;
  bones: Array<VisualBone>;
  /** Logical paths of the omf files this visual animates from. */
  motionRefs: Array<string>;
  /** Names of motions stored inside the visual itself, for a self animated model. */
  embeddedMotions: Array<string>;
  bufferLength: number;
};

/**
 * The slice of an index buffer that draws one detail level.
 *
 * Element offsets into the index buffer, not bytes, because that is what a draw call takes.
 */
export type VisualDrawRange = {
  start: number;
  count: number;
};

/**
 * Where one submesh's attributes sit inside the geometry buffer, and what to draw from them.
 *
 * Every section is a byte range into the one buffer the model ships as, so a consumer builds views
 * over it without copying. `indices` covers the whole index buffer, including the coarser detail
 * levels a progressive submesh carries; `draw_range` is the slice that renders the model at full
 * detail, already resolved so a consumer never has to pick.
 */
export type VisualGeometry = {
  vertexCount: number;
  indexCount: number;
  positions: VisualSection;
  normals: VisualSection;
  uvs: VisualSection;
  indices: VisualSection;
  drawRange: VisualDrawRange;
  /**
   * Detail levels of a progressive submesh, empty for a static one.
   *
   * Indices outside `draw_range` are validated only when a consumer decides to draw them, so a
   * detail level other than the first must be range checked before use.
   */
  windows: Array<VisualSlideWindow>;
  bounds: VisualBounds;
};

/**
 * Byte range of one packed attribute inside a visual's geometry buffer.
 *
 * Both values are byte counts rather than element counts, so a consumer builds a typed array view
 * directly from them. `byte_offset` is always a multiple of four, which `Float32Array` and
 * `Uint16Array` views both require; see [`crate::VisualBufferBuilder`].
 */
export type VisualSection = {
  byteOffset: number;
  byteLength: number;
};

/**
 * Why a submesh produced no geometry, graded so a caller does not read the message to find out.
 *
 * The distinction is what separates a gap in this crate's coverage from a file that contradicts
 * itself, which is the difference between a sweep noting something and a sweep failing.
 */
export type VisualSkipCause =
  /**
   * Geometry is stored in a form the packer does not handle, such as a shared vertex or index
   * container living outside the file.
   */
  | "unsupported"
  /** Geometry contradicts itself, such as a detail level reaching past the index buffer it indexes. */
  | "malformed";

/**
 * One progressive mesh detail level, mirroring `OgfSlideWindow` with renderer-facing names.
 *
 * Shipped in full even though only level zero is drawn today: the index buffer carries every level,
 * so withholding the table would make the coarser levels unreachable without a second read.
 */
export type VisualSlideWindow = {
  offset: number;
  triangleCount: number;
  vertexCount: number;
};

/** Enclosing sphere in three.js space. */
export type VisualSphere = {
  center: Vector3d;
  radius: number | null;
};

/** One drawable piece of a visual: a child of a skeleton, or a whole single level visual. */
export type VisualSubmesh = {
  index: number;
  modelType: number;
  modelTypeLabel: string;
  /**
   * X-Ray logical texture path, without an extension. A skeleton keeps these on its children rather
   * than at the top level, which is why a skeleton's own texture chunk is usually absent.
   */
  textureName: string | null;
  shaderName: string | null;
  content: VisualSubmeshContent;
};

/**
 * Whether a submesh produced drawable geometry, and why not when it did not.
 *
 * A child that cannot be packed is a value rather than an error so the rest of a model still
 * renders, and so the reason reaches the panel that lists it.
 */
export type VisualSubmeshContent =
  | { kind: "packed"; geometry: VisualGeometry }
  | { kind: "skipped"; cause: VisualSkipCause; reason: string };
