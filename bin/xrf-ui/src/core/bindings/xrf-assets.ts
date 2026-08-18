// Auto-generated rust bindings. Do not edit it manually.

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
