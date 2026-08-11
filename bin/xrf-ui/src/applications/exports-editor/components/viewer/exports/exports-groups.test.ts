import { describe, expect, it } from "@jest/globals";

import { ICallableExportDescriptor, IExportDescriptor, IValueExportDescriptor } from "@/lib/exports";

import { filterExportGroups, groupExports, ROOT_EXPORT_GROUP_ID } from "./exports-groups";

function callable(name: string, description: string | null = null): ICallableExportDescriptor {
  return {
    kind: "callable",
    name,
    description,
    parameters: [],
    returns: { typing: "void", description: null },
    source: { path: `${name}.ts`, line: 1, column: 1 },
  };
}

function value(name: string, description: string | null = null): IValueExportDescriptor {
  return {
    kind: "value",
    name,
    description,
    typing: "string",
    source: { path: `${name}.ts`, line: 1, column: 1 },
  };
}

describe("groupExports", () => {
  it("groups by the first dot and keeps root externs in a separate group", () => {
    const groups = groupExports([
      callable("xr_effects.run"),
      callable("start"),
      callable("dialogs_zaton.quest.answer"),
      callable("xr_effects.stop"),
    ]);

    expect(groups.map((group) => [group.id, group.label])).toEqual([
      [ROOT_EXPORT_GROUP_ID, "~"],
      ["group:namespace:dialogs_zaton", "dialogs_zaton"],
      ["group:namespace:xr_effects", "xr_effects"],
    ]);
    expect(groups[0]?.declarations.map((declaration) => declaration.name)).toEqual(["start"]);
    expect(groups[1]?.declarations.map((declaration) => declaration.name)).toEqual(["dialogs_zaton.quest.answer"]);
    expect(groups[2]?.declarations.map((declaration) => declaration.name)).toEqual([
      "xr_effects.run",
      "xr_effects.stop",
    ]);
  });

  it("filters globally and case-insensitively through every documentation field", () => {
    const callback: ICallableExportDescriptor = {
      ...callable("callbacks.run", "Runs a callback"),
      parameters: [{ name: "actor", typing: "game_object", description: "Target STALKER", isOptional: false }],
      returns: { typing: "boolean", description: "Whether it SUCCEEDED" },
    };
    const declarations: Array<IExportDescriptor> = [callback, value("settings", "Shared configuration")];
    const groups = groupExports(declarations);

    expect(filterExportGroups(groups, "CALLBACK")[0]?.declarations).toEqual([callback]);
    expect(filterExportGroups(groups, "stalker")[0]?.declarations).toEqual([callback]);
    expect(filterExportGroups(groups, "succeeded")[0]?.declarations).toEqual([callback]);
    expect(filterExportGroups(groups, "CONFIGURATION")[0]?.declarations[0]?.name).toBe("settings");
    expect(filterExportGroups(groups, "missing")).toEqual([]);
  });
});
