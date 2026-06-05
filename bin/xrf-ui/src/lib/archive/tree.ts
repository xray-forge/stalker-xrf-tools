import { TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";

import { Maybe } from "@/core/types/general";
import { IArchiveFileReplicationDescriptor } from "@/lib/archive/types";

export function parseTree(
  files: Array<IArchiveFileReplicationDescriptor>,
  separator: string
): Array<TreeViewDefaultItemModelProperties> {
  const node: TreeViewDefaultItemModelProperties = { id: "~", label: "root", children: [] };

  for (const file of files) {
    const path: string = file.name;
    const split: Array<string> = path.split(separator);

    createNode(split, node);
  }

  sortNode(node);

  return node.children!;
}

function createNode(path: Array<string>, parent: TreeViewDefaultItemModelProperties): void {
  const name: Maybe<string> = path.shift();

  if (!name) {
    return;
  }

  const element: Maybe<TreeViewDefaultItemModelProperties> = parent.children!.find(
    (element: TreeViewDefaultItemModelProperties) => {
      return element.label === name;
    }
  );

  if (element) {
    createNode(path, element);
  } else {
    const node: TreeViewDefaultItemModelProperties = {
      id: parent.id + "\\" + name,
      label: name,
      children: [],
    };

    parent.children!.push(node);

    if (path.length) {
      createNode(path, node);
    }
  }
}

function sortNode(node: TreeViewDefaultItemModelProperties): void {
  for (const children of node.children!) {
    if (children.children?.length) {
      sortNode(children);
    }
  }

  node.children!.sort((a, b) => {
    if ((!a.children!.length && !b.children!.length) || (a.children!.length && b.children!.length)) {
      return a.label.localeCompare(b.label);
    }

    return (b.children?.length ?? 0) - (a.children?.length ?? 0);
  });
}
