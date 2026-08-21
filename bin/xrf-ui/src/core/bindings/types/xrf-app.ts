// Auto-generated rust bindings. Do not edit it manually.

import { InventorySpriteDescriptor } from "@/core/bindings/types/xrf-texture";
import { XrayAsset } from "@/core/bindings/types/xrf-vfs";
import { VisualDescription } from "@/core/bindings/types/xrf-visual";

/** The X-Ray source parameters carried in a sound's first vorbis comment. */
export type ArchiveAudioParameters = {
  minDistance: number | null;
  maxDistance: number | null;
  baseVolume: number | null;
  gameType: number;
  maxAiDistance: number | null;
};

export type ArchiveAudioPreview = {
  name: string;
  channels: number;
  sampleRate: number;
  /** Absent for a sound carrying no recognized X-Ray comment, where the engine uses its own defaults. */
  parameters: ArchiveAudioParameters | null;
  /** The ogg bytes as stored, base64 encoded. The webview decodes vorbis itself. */
  base64: string;
};

export type ArchiveImagePreview = {
  name: string;
  width: number;
  height: number;
  /** PNG bytes, base64 encoded so the webview can use them directly as an image source. */
  base64: string;
};

export type EquipmentSpriteMetadata = {
  path: string;
  name: string;
  systemLtxPath: string;
  equipmentDescriptors: Array<InventorySpriteDescriptor>;
};

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

/** Where a visual is read from. */
export type VisualSource =
  /** A loose `.ogf` file on disk. */
  { kind: "file"; path: string };
