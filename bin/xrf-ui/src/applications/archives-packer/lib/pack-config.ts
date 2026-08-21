import {
  ArchivePackConfig,
  ArchivePackFolder,
  ArchivePackMode,
  ArchiveVolumeExtension,
} from "@/core/bindings/types/xrf-archive";
import { BYTES_PER_MEGABYTE } from "@/lib/memory/size";
import { Nullable } from "@/lib/types/general";

/**
 * The variants of the packer's own enums, named rather than spelled out wherever one is compared.
 *
 * Typed as a complete mapping of the generated union, so a variant renamed or dropped in `xrf-archive`
 * fails to compile here instead of silently becoming a comparison that never matches.
 */
export const ARCHIVE_VOLUME_EXTENSION: { [K in ArchiveVolumeExtension]: K } = {
  Db: "Db",
  Xdb: "Xdb",
};

/** The extension a volume actually carries, mirroring `ArchiveVolumeExtension::as_str`. */
export const ARCHIVE_VOLUME_SUFFIX: { [K in ArchiveVolumeExtension]: string } = {
  Db: "db",
  Xdb: "xdb",
};

export const ARCHIVE_PACK_MODE: { [K in ArchivePackMode]: K } = {
  Compress: "Compress",
  Store: "Store",
};

/** Header key the engine reads to decide where an archive mounts. */
export const HEADER_ENTRY_POINT: string = "entry_point";

/** Header key the engine reads to decide whether to mount the archive at startup at all. */
export const HEADER_AUTO_LOAD: string = "auto_load";

/** Keys the editor gives a control of their own, so the rest can be listed as custom values. */
export const RESERVED_HEADER_KEYS: ReadonlyArray<string> = [HEADER_ENTRY_POINT, HEADER_AUTO_LOAD];

/** What a packed `gamedata` tree mounts as, and the value the engine expects for one. */
export const DEFAULT_ENTRY_POINT: string = "$fs_root$\\gamedata\\";

/**
 * What the editor opens on before the packer answers with its own defaults.
 */
export const FALLBACK_PACK_CONFIG: ArchivePackConfig = {
  source: "",
  destination: "",
  name: "gamedata",
  includeFiles: [],
  includeFolders: [],
  excludeFolders: [],
  excludeExtensions: [],
  isWithSkipList: true,
  header: `[header]\r\nauto_load = true\r\nentry_point = ${DEFAULT_ENTRY_POINT}\r\n`,
  mode: ARCHIVE_PACK_MODE.Compress,
  maxVolumeSize: 1900 * BYTES_PER_MEGABYTE,
  volumeExtension: ARCHIVE_VOLUME_EXTENSION.Db,
};

/**
 * Read one key out of the header text.
 *
 * The header is carried verbatim because the archive stores it that way, so the editor parses it only
 * far enough to show and change a single line.
 */
export function readHeaderValue(header: Nullable<string>, key: string): Nullable<string> {
  if (!header) {
    return null;
  }

  for (const line of header.split(/\r?\n/)) {
    const [name, ...rest] = line.split("=");

    if (rest.length && name.trim() === key) {
      return rest.join("=").trim();
    }
  }

  return null;
}

/**
 * Replace or append one key in the header text, keeping every other line as it was.
 *
 * An empty value removes the key, and removing the last key removes the header, because an archive with
 * an empty header section is not the same as one with none.
 */
export function writeHeaderValue(header: Nullable<string>, key: string, value: string): Nullable<string> {
  const lines: Array<string> = (header ?? "")
    .split(/\r?\n/)
    .filter((line) => line.trim() && !line.trim().startsWith("["))
    .filter((line) => line.split("=")[0]?.trim() !== key);

  if (value.trim()) {
    lines.push(`${key} = ${value.trim()}`);
  }

  if (!lines.length) {
    return null;
  }

  return `[header]\r\n${lines.join("\r\n")}\r\n`;
}

/**
 * Read one header key as a flag.
 *
 * Accepts what the engine's own LTX reader accepts rather than only the literal it writes, so a header
 * that came from a hand-written configuration reads the same way the game will read it.
 */
export function readHeaderFlag(header: Nullable<string>, key: string): boolean {
  const value: Nullable<string> = readHeaderValue(header, key);

  return value !== null && ["true", "on", "yes", "1"].includes(value.toLowerCase());
}

/** Written out in full rather than dropped when false, because a missing key and a false one read alike. */
export function writeHeaderFlag(header: Nullable<string>, key: string, isEnabled: boolean): Nullable<string> {
  return writeHeaderValue(header, key, isEnabled ? "true" : "false");
}

/** Every header line except the section marker, for listing what a header carries. */
export function readHeaderEntries(header: Nullable<string>): Array<[string, string]> {
  if (!header) {
    return [];
  }

  return header
    .split(/\r?\n/)
    .map((line) => line.split("="))
    .filter((parts) => parts.length > 1)
    .map((parts) => [parts[0].trim(), parts.slice(1).join("=").trim()] as [string, string]);
}

export function withFolderAt(
  folders: Array<ArchivePackFolder>,
  index: number,
  patch: Partial<ArchivePackFolder>
): Array<ArchivePackFolder> {
  return folders.map((folder, at) => (at === index ? { ...folder, ...patch } : folder));
}

export function withoutAt<T>(items: Array<T>, index: number): Array<T> {
  return items.filter((_, at) => at !== index);
}

export function withValueAt(items: Array<string>, index: number, value: string): Array<string> {
  return items.map((item, at) => (at === index ? value : item));
}

/**
 * Whether the configuration selects anything specific.
 *
 * Selecting nothing is not an empty archive: the packer reads it as the whole source directory, which is
 * worth saying out loud in the editor rather than leaving the sections looking unfinished.
 */
export function isWholeDirectory(config: ArchivePackConfig): boolean {
  return !config.includeFolders.length && !config.includeFiles.length;
}
