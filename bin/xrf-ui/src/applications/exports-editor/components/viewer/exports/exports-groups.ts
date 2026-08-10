import { IExportDescriptor } from "@/lib/exports";

export const ROOT_EXPORT_GROUP_ID: string = "root";

export interface IExportGroup {
  id: string;
  label: string;
  declarations: Array<IExportDescriptor>;
}

/** Group externs by the namespace before their first dot. */
export function groupExports(declarations: ReadonlyArray<IExportDescriptor>): Array<IExportGroup> {
  const groups: Map<string, IExportGroup> = new Map();

  for (const declaration of declarations) {
    const separator: number = declaration.name.indexOf(".");
    const isRoot: boolean = separator < 0;
    const namespace: string = isRoot ? "" : declaration.name.slice(0, separator);
    const id: string = isRoot ? ROOT_EXPORT_GROUP_ID : `namespace:${namespace}`;
    const group: IExportGroup = groups.get(id) ?? {
      id,
      label: isRoot ? "Root" : namespace,
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
      declarations: group.declarations.sort((left: IExportDescriptor, right: IExportDescriptor) =>
        left.name.localeCompare(right.name)
      ),
    }));
}
