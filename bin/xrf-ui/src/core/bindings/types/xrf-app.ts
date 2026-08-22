// Auto-generated rust bindings. Do not edit it manually.

import { InventorySpriteDescriptor } from "@/core/bindings/types/xrf-texture";
import { VisualDependencies, VisualDescription } from "@/core/bindings/types/xrf-visual";

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

/**
 * Where a caller wants an asset looked for, named rather than handed over.
 *
 * Self-describing on purpose: a world is identified by what it is made of, never by a handle the backend issued. A
 * webview reload therefore loses nothing, and a surface that did not open a world can still address assets in it —
 * which is what lets one plugin's selection be read by another's preview.
 *
 * The subject asset is not part of a spec. A command that already names one — a model being opened — passes it
 * separately, and its own tree and installation are searched ahead of these roots.
 */
export type AssetWorldSpec = {
  /** Roots searched in the order given. */
  roots: Array<string>;
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
 * The source travels back so a frontend that reloaded knows what to ask geometry for, without having to remember
 * anything of its own across the reload.
 */
export type SelectedVisualDescription = {
  source: VisualSource;
  description: VisualDescription;
  dependencies: VisualDependencies;
};

/** Where a visual is read from. */
export type VisualSource =
  /** A loose `.ogf` file on disk. */
  { kind: "file"; path: string };
