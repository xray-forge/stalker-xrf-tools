// Auto-generated rust bindings. Do not edit it manually.

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
 * An owned result of locating an asset.
 *
 * Unlike `XrayAsset`, this type can be stored or sent over IPC. It preserves the engine path and source-reported
 * container for either a loose or archived asset.
 */
export type XrayAssetLocation = {
  /** Lower-case, backslash-separated engine identity, including the mount's logical base. */
  logicalPath: string;
  /** Physical container reported by the source that resolved the asset. */
  container: XrayAssetContainer;
};
