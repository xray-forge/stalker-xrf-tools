import { ICallableExportDescriptor, IExportDescriptor } from "@/lib/exports";

export const ROOT_EXPORT_GROUP_ID: string = "group:root";

export interface IExportGroup {
  id: string;
  label: string;
  declarations: Array<IExportDescriptor>;
}

export interface IExportTreeItem {
  id: string;
  label: string;
  kind: "group" | "declaration";
  children?: Array<IExportTreeItem>;
}

/** Group externs by the namespace before their first dot. */
export function groupExports(declarations: ReadonlyArray<IExportDescriptor>): Array<IExportGroup> {
  const groups: Map<string, IExportGroup> = new Map();

  for (const declaration of declarations) {
    const separator: number = declaration.name.indexOf(".");
    const isRoot: boolean = separator < 0;
    const namespace: string = isRoot ? "" : declaration.name.slice(0, separator);
    const id: string = isRoot ? ROOT_EXPORT_GROUP_ID : `group:namespace:${namespace}`;
    const group: IExportGroup = groups.get(id) ?? {
      id,
      label: isRoot ? "~" : namespace,
      declarations: [],
    };

    group.declarations.push(declaration);
    groups.set(id, group);
  }

  return Array.from(groups.values())
    .sort((left: IExportGroup, right: IExportGroup) => {
      if (left.id === ROOT_EXPORT_GROUP_ID) {
        return -1;
      }

      if (right.id === ROOT_EXPORT_GROUP_ID) {
        return 1;
      }

      return left.label.localeCompare(right.label);
    })
    .map((group: IExportGroup) => ({
      ...group,
      declarations: [...group.declarations].sort((left: IExportDescriptor, right: IExportDescriptor) =>
        left.name.localeCompare(right.name)
      ),
    }));
}

export function filterExportGroups(groups: ReadonlyArray<IExportGroup>, query: string): Array<IExportGroup> {
  const normalized: string = query.trim().toLocaleLowerCase();

  if (!normalized) {
    return [...groups];
  }

  return groups
    .map((group: IExportGroup) => ({
      ...group,
      declarations: group.declarations.filter((declaration: IExportDescriptor) =>
        getExportSearchText(declaration).includes(normalized)
      ),
    }))
    .filter((group: IExportGroup) => group.declarations.length > 0);
}

export function exportDeclarationItemId(name: string): string {
  return `declaration:${name}`;
}

export function exportGroupsToTree(groups: ReadonlyArray<IExportGroup>): Array<IExportTreeItem> {
  return groups.map((group: IExportGroup) => ({
    id: group.id,
    label: `${group.label} (${group.declarations.length})`,
    kind: "group",
    children: group.declarations.map((declaration: IExportDescriptor) => ({
      id: exportDeclarationItemId(declaration.name),
      label: declaration.name,
      kind: "declaration",
    })),
  }));
}

function getExportSearchText(declaration: IExportDescriptor): string {
  const documentation: Array<string> = [declaration.name, declaration.description ?? ""];

  if (declaration.kind === "callable") {
    const callable: ICallableExportDescriptor = declaration;

    documentation.push(callable.returns.description ?? "");
    documentation.push(...callable.parameters.map((parameter) => parameter.description ?? ""));
  }

  return documentation.join("\n").toLocaleLowerCase();
}
