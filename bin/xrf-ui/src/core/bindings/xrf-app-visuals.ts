// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  /** Drop the selected visual and its packed geometry. */
  closeModel: () => __TAURI_INVOKE<null>("plugin:visuals|close_model"),
  /**
   * What the viewer had selected, or null when nothing is open.
   *
   * This is the rehydration probe: a reloaded frontend asks what is selected and then asks for that
   * source's geometry, so the selection survives a reload without the frontend storing anything.
   */
  getModel: () =>
    __TAURI_INVOKE<{
      source: VisualSource;
      description: VisualDescription;
      textures: Array<SubmeshTexture>;
    } | null>("plugin:visuals|get_model"),
  /**
   * Select a visual and return what it contains.
   *
   * Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather
   * than repeating it. The bytes are not returned: a typed command cannot carry them, which is why they
   * are read separately.
   *
   * Texture references are resolved here too, so the frontend learns in one round trip which submeshes
   * have a texture to fetch and which do not. `fallback_root` is the configured project's gamedata path,
   * which only the frontend knows; it is used only when the visual's own tree does not answer.
   */
  openModel: (source: VisualSource, fallbackRoot: string | null) =>
    __TAURI_INVOKE<SelectedVisualDescription>("plugin:visuals|open_model", { source, fallbackRoot }),
};

/* Types */
/**
 * What the viewer is showing, paired with where it came from.
 *
 * The source travels back so a frontend that reloaded knows what to ask geometry for, without having
 * to remember anything of its own across the reload.
 *
 * Texture resolution rides here rather than on `VisualDescription` because it is a fact about this
 * machine's filesystem, while a description is a fact about the file - which is also why `xrf-visual`
 * has no filesystem surface to resolve with.
 */
export type SelectedVisualDescription = {
  source: VisualSource;
  description: VisualDescription;
  textures: Array<SubmeshTexture>;
};

/**
 * One submesh's texture reference paired with what became of it.
 *
 * The reference stays outside the outcome because it is what `read_texture` is addressed by, and it is the same string
 * whether resolution succeeded, substituted or failed.
 */
export type SubmeshTexture = {
  /** Index of the submesh this belongs to, so a consumer pairs them without relying on order. */
  submeshIndex: number;
  /** X-Ray logical path the submesh declares, absent when it declares none. */
  reference: string | null;
  resolution: SubmeshTextureResolution;
};

/**
 * What resolving one submesh's texture reference produced.
 *
 * A tagged enum rather than a struct of options, so the impossible combinations - a resolved texture with no
 * location, a root-less lookup that still found a file - cannot be constructed or arrive on the wire.
 *
 * The located variants carry an [`XrayAssetLocation`] rather than their own path fields, so they gain archive-backed
 * assets when that type does.
 */
export type SubmeshTextureResolution =
  /** The submesh declares no texture at all, which is normal for a skeleton's own record. */
  | { kind: "none" }
  /**
   * Nothing above the visual looks like an X-Ray root and no project root was offered, so no lookup was attempted.
   *
   * Distinct from `Missing` on purpose: it says the question could not be asked, not that the answer was no.
   */
  | { kind: "noRoot" }
  /** The reference resolved inside a root. */
  | { kind: "resolved"; location: XrayAssetLocation }
  /** The reference resolved nowhere, so the engine's dummy stands in - as it does in game. */
  | { kind: "substituted"; location: XrayAssetLocation }
  /** Neither the reference nor the dummy resolved, so there is nothing to show. */
  | { kind: "missing"; root: string };

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

/**
 * Where a visual is read from.
 *
 * An enum from the start because reading a visual out of an opened archive is the next source, and it
 * differs only in how bytes are obtained. Keeping the shape means that arrives as a variant rather
 * than as a second pair of commands.
 */
export type VisualSource =
  /** A loose `.ogf` file on disk. */
  { kind: "file"; path: string };

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

/**
 * Where an asset was found, detached from the index that found it.
 *
 * [`XrayAsset`] borrows from its index, so it cannot be parked in state, sent over IPC or kept past the lookup. This is
 * the same three facts owned: which root answered, which file inside it, and the engine identity it answers to.
 *
 * Root and relative path stay separate rather than joined, because "which root did this come from" is the question an
 * overlay tree makes interesting, and joining them throws it away.
 *
 * When archive-backed assets arrive this is the type that gains a container, so a consumer reading a located asset does
 * not change shape when the bytes start coming out of a `.db`.
 */
export type XrayAssetLocation = {
  /** Indexed root this resolved against. */
  root: string;
  /** Physical path inside that root. */
  relativePath: string;
  /** Lower-case, backslash-separated engine identity. */
  logicalPath: string;
};
