import { Optional } from "@/core/types/general";
import { ArchiveFileDescriptor } from "@/lib/bindings/xray-archive";

/**
 * Whether an archived file would be written when its directory is extracted.
 *
 * Mirrors `ArchiveProject::extract_folder` on the rust side, including both of its skips. Counting with
 * a plain `startsWith` here instead would promise more files than the backend writes, and would let
 * `configs` swallow `configs_backup`.
 */
export function isUnderArchiveDirectory(descriptor: ArchiveFileDescriptor, prefix: string): boolean {
  // Directory entries and empty entries are never written out.
  if (!descriptor.sizeReal || /[\\/]$/.test(descriptor.name)) {
    return false;
  }

  if (!prefix) {
    return true;
  }

  const name: string = descriptor.name.toLowerCase();
  const normalized: string = prefix.replace(/[\\/]+$/, "").toLowerCase();

  return name.length > normalized.length && name.startsWith(normalized) && /[\\/]/.test(name[normalized.length]);
}

export interface IArchiveDirectoryTreeItem {
  id: string;
  label: string;
  path: string;
  kind: "directory";
  children: Array<IArchiveTreeItem>;
}

export interface IArchiveFileTreeItem {
  id: string;
  label: string;
  path: string;
  kind: "file";
  descriptor: ArchiveFileDescriptor;
}

export type IArchiveTreeItem = IArchiveDirectoryTreeItem | IArchiveFileTreeItem;

/**
 * Build a directory-first explorer tree from effective archive file descriptors.
 *
 * @param files - Effective archive files to attach to leaf nodes.
 * @param separator - Separator used by the archive-relative file paths.
 * @returns Sorted root-level tree items with descriptors attached to file leaves.
 */
export function parseTree(files: Array<ArchiveFileDescriptor>, separator: string): Array<IArchiveTreeItem> {
  const root: IArchiveDirectoryTreeItem = {
    id: "directory:~",
    label: "root",
    path: "",
    kind: "directory",
    children: [],
  };

  for (const descriptor of files) {
    appendFile(root, descriptor.name.split(separator), descriptor, separator);
  }

  sortTree(root.children);

  return root.children;
}

/**
 * Append one archive file to a mutable directory tree.
 *
 * @param parent - Directory node that receives the next path segment.
 * @param remainingPath - Mutable path segments still to consume for the file.
 * @param descriptor - File descriptor attached to the resulting leaf node.
 * @param separator - Separator used to reconstruct each canonical node path.
 */
function appendFile(
  parent: IArchiveDirectoryTreeItem,
  remainingPath: Array<string>,
  descriptor: ArchiveFileDescriptor,
  separator: string
): void {
  const name: Optional<string> = remainingPath.shift();

  if (!name) {
    return;
  }

  const path: string = parent.path ? `${parent.path}${separator}${name}` : name;

  if (!remainingPath.length) {
    parent.children.push({ id: `file:${path}`, label: name, path, kind: "file", descriptor });

    return;
  }

  const existing: Optional<IArchiveTreeItem> = parent.children.find(
    (child: IArchiveTreeItem) => child.kind === "directory" && child.label === name
  );
  const directory: IArchiveDirectoryTreeItem =
    existing?.kind === "directory"
      ? existing
      : { id: `directory:${path}`, label: name, path, kind: "directory", children: [] };

  if (!existing) {
    parent.children.push(directory);
  }

  appendFile(directory, remainingPath, descriptor, separator);
}

/**
 * Sort a mutable tree recursively with directories before files and labels in locale order.
 *
 * @param items - Tree items to sort in place.
 * @returns {void} Nothing.
 */
function sortTree(items: Array<IArchiveTreeItem>): void {
  for (const item of items) {
    if (item.kind === "directory") {
      sortTree(item.children);
    }
  }

  items.sort((first: IArchiveTreeItem, second: IArchiveTreeItem) => {
    if (first.kind !== second.kind) {
      return first.kind === "directory" ? -1 : 1;
    }

    return first.label.localeCompare(second.label);
  });
}
