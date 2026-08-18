import { CompressedPixelFormat, CompressedTexture, LinearFilter } from "three";
import { DDS, DDSLoader } from "three/examples/jsm/loaders/DDSLoader.js";

import { SubmeshTexture, SubmeshTextureResolution } from "@/core/bindings/xrf-app-visuals";
import { Nullable } from "@/lib/types/general";

/** Shared parser, since `DDSLoader.parse` keeps no state between calls and constructing one per texture is waste. */
const DDS_LOADER: DDSLoader = new DDSLoader();

/**
 * Why a submesh ended up without a texture on screen, or that it has one.
 */
export enum EVisualTextureState {
  /** The submesh declares no texture, which is normal for a skeleton's own record. */
  ABSENT = "absent",
  /** Bytes are still on the way. */
  LOADING = "loading",
  /** Uploaded and applied. */
  APPLIED = "applied",
  /** Located, but stored in a format three.js cannot upload. */
  UNSUPPORTED_FORMAT = "unsupportedFormat",
  /** Nothing to load: no root was found, or neither the reference nor the engine's dummy resolved. */
  UNRESOLVED = "unresolved",
  /** Located, but reading or parsing the file failed. */
  FAILED = "failed",
}

/**
 *  What became of one submesh's texture on the frontend, paired with what the backend resolved.
 */
export interface IVisualTextureStatus {
  submeshIndex: number;
  state: EVisualTextureState;
  /** Present when the state is `FAILED`, so a panel can say why rather than only that. */
  reason: Nullable<string>;
}

/**
 * Whether a resolution gives the frontend anything to fetch.
 *
 * A substituted reference counts: the engine's dummy is a real file and rendering it is what the game does, which
 * is the point of substituting rather than leaving the submesh blank.
 */
export function isLoadableResolution(resolution: SubmeshTextureResolution): boolean {
  return resolution.kind === "resolved" || resolution.kind === "substituted";
}

/**
 * Submeshes worth fetching bytes for, which is those with both a reference and a located file.
 */
export function toLoadableTextures(textures: Array<SubmeshTexture>): Array<SubmeshTexture & { reference: string }> {
  return textures.filter(
    (texture): texture is SubmeshTexture & { reference: string } =>
      texture.reference !== null && isLoadableResolution(texture.resolution)
  );
}

/**
 * The state a submesh starts in, before any bytes are asked for.
 */
export function toInitialTextureState(texture: SubmeshTexture): EVisualTextureState {
  if (texture.reference === null) {
    return EVisualTextureState.ABSENT;
  }

  return isLoadableResolution(texture.resolution) ? EVisualTextureState.LOADING : EVisualTextureState.UNRESOLVED;
}

/**
 * Turn DDS bytes into an uploadable texture, or say that three.js cannot.
 *
 * `DDSLoader` refuses two ways and both look the same from outside: an unknown `DXGI_FORMAT` under a DX10 header
 * logs to the console and falls through, and an uncompressed layout whose channel masks are not BGRA matches neither
 * of its two uncompressed branches. Either way it returns a parse with a null format, so checking that covers both -
 * and covers the next format it has not learnt yet. Both occur in the reference trees: `BC7_UNorm` in Gunslinger and
 * `A8B8G8R8` in Anomaly.
 *
 * Assembly follows `CompressedTextureLoader`'s own single-file path rather than improvising, because one of its
 * steps is load-bearing: a texture carrying no mip chain must drop to `LinearFilter`, or webgl samples an incomplete
 * texture and renders black. Not an edge case here - 1,805 of Anomaly's 2,197 distinct textures ship without mips.
 *
 * @returns The texture, or null when three.js cannot upload this file.
 */
export function createDdsTexture(bytes: ArrayBuffer): Nullable<CompressedTexture> {
  const parsed: DDS = DDS_LOADER.parse(bytes, true);

  // The declared type is not nullable, but the parser initialises `format` to null and leaves it there when it refuses.
  if (parsed.format === null || parsed.mipmaps.length === 0) {
    return null;
  }

  // A cubemap needs its faces split apart, which no model texture requires; rendering it flat would show one face
  // stretched over the mesh, so it is refused rather than guessed at.
  if (parsed.isCubemap) {
    return null;
  }

  const texture: CompressedTexture = new CompressedTexture(
    parsed.mipmaps,
    parsed.width,
    parsed.height,
    // `DDSLoader` reports `RGBAFormat` for an uncompressed file, which the typings do not admit here even though
    // three's own `CompressedTextureLoader` assigns exactly that to a `CompressedTexture`.
    parsed.format as CompressedPixelFormat
  );

  if (parsed.mipmapCount === 1) {
    texture.minFilter = LinearFilter;
  }

  texture.needsUpdate = true;

  return texture;
}
