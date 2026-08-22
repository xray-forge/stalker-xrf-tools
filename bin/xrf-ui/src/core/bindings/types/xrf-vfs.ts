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

/**
 * What one reference lookup came to.
 *
 * A fact about a lookup, not about the kind of thing looked up: a texture, a motion set and a level asset all end in
 * one of these states, so a consumer renders one shape and a domain crate pairs the outcome with its own reference
 * identity rather than defining its own vocabulary.
 *
 * A missing asset is a state rather than an error, because it is one in the engine too — `Missing` carries where the
 * probe looked so a report can say that instead of only that nothing was found.
 */
export type XrayResolution =
  /**
   * The reference itself resolved.
   *
   * `assets` is never empty, and holds more than one entry only for a mask — a motion reference may name a set.
   */
  | { kind: "resolved"; step: string; assets: Array<XrayAsset> }
  /**
   * The reference did not resolve, but the fallback the caller offered did.
   *
   * Substitution is engine behavior a caller opts into per kind, so the fallback reference travels back: reporting the
   * asset alone would show a located texture while hiding that it is not the requested one.
   */
  | { kind: "substituted"; step: string; fallback: string; assets: Array<XrayAsset> }
  /**
   * Nothing resolved, across every step of the probe.
   *
   * `roots` is every source searched, in probe order and without duplicates.
   */
  | { kind: "missing"; roots: Array<string> }
  /**
   * There was nothing to search: the probe had no step, or no step selected a mounted source.
   *
   * Distinct from `Missing` because the question could not be asked rather than the answer being no, which is
   * the difference between an unconfigured project and an absent asset.
   */
  | { kind: "noScope" }
  /**
   * The reference could not be turned into a lookup at all, so none was attempted.
   *
   * Engine text is untrusted: a mesh header may hold a name no logical path can be made of. Folding that into `Missing`
   * would report a garbage reference as an absent asset, and substituting for it would report it as a present one.
   */
  | { kind: "rejected"; reason: string };
