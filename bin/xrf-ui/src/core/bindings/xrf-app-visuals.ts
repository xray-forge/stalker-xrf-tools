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
 */
export type SelectedVisualDescription = {
  source: VisualSource;
  description: VisualDescription;
  textures: Array<SubmeshTexture>;
};

/**
 * One submesh texture reference and its resolution outcome.
 *
 * When present, the reference is retained for `read_texture` regardless of the resolution outcome.
 */
export type SubmeshTexture = {
  /** Submesh index used to pair this outcome without relying on response order. */
  submeshIndex: number;
  /** X-Ray texture reference declared by the submesh, or `None` when omitted. */
  reference: string | null;
  resolution: SubmeshTextureResolution;
};

/**
 * The outcome of resolving one submesh texture reference.
 *
 * Separate variants distinguish an omitted reference, an unavailable search root, a missing texture, and a located
 * asset. Located assets use `XrayAsset` to describe either a directory or archive container.
 */
export type SubmeshTextureResolution =
  /** The submesh declares no texture, as is normal for a skeleton root record. */
  | { kind: "none" }
  /** No visual or fallback root was available, so no lookup was attempted. */
  | { kind: "noRoot" }
  /** The reference resolved within the search scope. */
  | { kind: "resolved"; location: XrayAsset }
  /** The reference was absent, but the engine's fallback texture resolved. */
  | { kind: "substituted"; location: XrayAsset }
  /**
   * Neither the reference nor the engine's fallback texture resolved.
   *
   * `roots` lists every source searched by the scope.
   */
  | { kind: "missing"; roots: Array<string> };

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
 * builds views from the byte ranges each submesh carries. The reported total buffer length makes a
 * mismatched description and buffer detectable.
 */
export type VisualDescription = {
  version: number;
  modelType: number;
  modelTypeLabel: string;
  shaderId: number;
  /** Source object the OGF was built from, when the file records one. */
  sourceFile: string | null;
  /** Extent the header declares, converted into three.js space for comparison with the computed extent. */
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
 * levels a progressive submesh carries; the resolved draw range renders the model at full
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
   * Indices outside the resolved draw range are validated only when a consumer decides to draw them, so a
   * detail level other than the first must be range checked before use.
   */
  windows: Array<VisualSlideWindow>;
  bounds: VisualBounds;
};

/**
 * Byte range of one packed attribute inside a visual's geometry buffer.
 *
 * Both values are byte counts rather than element counts, so a consumer builds a typed array view
 * directly from them. The packer aligns every offset to four bytes for `Float32Array` and
 * `Uint16Array` views.
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

/** Where a visual is read from. */
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
 * One asset a mount resolved: its engine identity plus the container it came out of.
 *
 * Owned rather than borrowed, so it can be stored, sorted or sent over IPC — which is what an editor that mounts and
 * writes needs, and why nothing borrowed reaches past this crate.
 */
export type XrayAsset = {
  /** Lower-case, backslash-separated engine identity, including the mount's logical base. */
  logicalPath: XrayPath;
  /** Physical container reported by the source that resolved the asset. */
  container: XrayAssetContainer;
};

/**
 * The physical container of a located asset.
 *
 * Separate variants prevent callers from treating an archived entry as a loose file with a usable filesystem path.
 */
export type XrayAssetContainer =
  /** A loose file, preserving its root so consumers can identify the winning overlay. */
  | { kind: "directory"; root: string; relativePath: string }
  /** An entry inside the archive volume set at `path`. */
  | { kind: "archive"; path: string };

/**
 * An X-Ray logical path: lower case, backslash separated, with no empty, `.` or `..` component.
 *
 * This is an engine identity, not a location on disk. The asset it names may sit inside an archive and have no file at
 * all, so the type deliberately does not implement `AsRef<Path>` — handing one to host I/O must not compile. Read it
 * through an [`crate::XrayVfs`], and ask [`crate::XrayAsset::physical_path`] when a real file is genuinely
 * required.
 *
 * Being separator-explicit is what makes it portable: it splits on `\` itself rather than deferring to
 * `std::path`, so `parent` and `file_name` answer the same on Linux as on Windows, where a `std::path::Path`
 * would treat the whole thing as one component.
 *
 * Serialized and typed transparently as its string form, so an engine path crosses IPC as the text the engine uses.
 */
export type XrayPath = string;
