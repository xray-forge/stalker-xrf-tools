import { Nullable, Optional } from "@/core/types/general";
import { IArchiveFileDescriptor } from "@/lib/archive/types";

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
  descriptor: IArchiveFileDescriptor;
}

export type IArchiveTreeItem = IArchiveDirectoryTreeItem | IArchiveFileTreeItem;

export interface IFilteredArchiveTree {
  items: Array<IArchiveTreeItem>;
  expandedItems: Array<string>;
}

/**
 * Build a directory-first explorer tree from effective archive file descriptors.
 *
 * @param files - Effective archive files to attach to leaf nodes.
 * @param separator - Separator used by the archive-relative file paths.
 * @returns Sorted root-level tree items with descriptors attached to file leaves.
 */
export function parseTree(files: Array<IArchiveFileDescriptor>, separator: string): Array<IArchiveTreeItem> {
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
 * Filter an archive tree by canonical path while retaining the hierarchy needed to reach each match.
 *
 * @param items - Root-level archive tree items to search.
 * @param rawQuery - Case-insensitive path query; surrounding whitespace is ignored.
 * @returns The filtered tree and directory identifiers that should be expanded to reveal its matches.
 */
export function filterArchiveTree(items: Array<IArchiveTreeItem>, rawQuery: string): IFilteredArchiveTree {
  const query: string = rawQuery.trim().toLocaleLowerCase();

  if (!query) {
    return { items, expandedItems: [] };
  }

  const expandedItems = new Set<string>();
  const filteredItems: Array<IArchiveTreeItem> = [];

  for (const item of items) {
    const filtered: IArchiveTreeItem | null = filterItem(item, query, expandedItems);

    if (filtered) {
      filteredItems.push(filtered);
    }
  }

  return { items: filteredItems, expandedItems: Array.from(expandedItems) };
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
  descriptor: IArchiveFileDescriptor,
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

/**
 * Retain one matching tree branch and record directories that expose its matches.
 *
 * @param item - Tree item to compare with the normalized query.
 * @param query - Trimmed lowercase path query.
 * @param expandedItems - Mutable accumulator for directory identifiers that should be expanded.
 * @returns The retained tree item, including filtered descendants, or null when the branch does not match.
 */
function filterItem(item: IArchiveTreeItem, query: string, expandedItems: Set<string>): Nullable<IArchiveTreeItem> {
  const isMatch: boolean = item.path.toLocaleLowerCase().includes(query);

  if (item.kind === "file") {
    return isMatch ? item : null;
  }

  if (isMatch) {
    expandedItems.add(item.id);

    return item;
  }

  const children: Array<IArchiveTreeItem> = item.children
    .map((child: IArchiveTreeItem) => filterItem(child, query, expandedItems))
    .filter((child: Nullable<IArchiveTreeItem>): child is IArchiveTreeItem => child !== null);

  if (!children.length) {
    return null;
  }

  expandedItems.add(item.id);

  return { ...item, children };
}
