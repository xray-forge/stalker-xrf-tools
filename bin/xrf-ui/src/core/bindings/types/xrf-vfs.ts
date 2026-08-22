// Auto-generated rust bindings. Do not edit it manually.

/**
 * One asset a mount resolved: its engine identity plus the container it came out of.
 *
 * Owned rather than borrowed, so it can be stored, sorted or sent over IPC — which is what an editor that mounts and
 * writes needs, and why nothing borrowed reaches past this crate.
 */
export type XrayAsset = {
  /** Lower-case, backslash-separated engine identity, including the mount's logical base. */
  logicalPath: XrayLogicalPath;
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
 * through an [`crate::XrayVfs`], and ask [`crate::XrayAsset::to_physical_path`] when a real file is genuinely
 * required.
 *
 * Being separator-explicit is what makes it portable: it splits on `\` itself rather than deferring to
 * `std::path`, so `parent` and `file_name` answer the same on Linux as on Windows, where a `std::path::Path`
 * would treat the whole thing as one component.
 *
 * Serialized and typed transparently as its string form, so an engine path crosses IPC as the text the engine uses.
 */
export type XrayLogicalPath = string;
