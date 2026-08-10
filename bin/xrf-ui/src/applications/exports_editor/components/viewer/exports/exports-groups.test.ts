import { describe, expect, it } from "@jest/globals";

import { IExportDescriptor } from "@/lib/exports";

import { groupExports, ROOT_EXPORT_GROUP_ID } from "./exports-groups";

function descriptor(name: string): IExportDescriptor {
  return {
    filename: `${name}.ts`,
    name,
    comment: null,
    parameters: [],
    typing: null,
    line: 1,
    col: 1,
  };
}

describe("groupExports", () => {
  it("groups by the first dot and keeps root externs in a separate tab", () => {
    const groups = groupExports([
      descriptor("xr_effects.run"),
      descriptor("start"),
      descriptor("dialogs_zaton.quest.answer"),
      descriptor("xr_effects.stop"),
    ]);

    expect(groups.map((group) => [group.id, group.label])).toEqual([
      [ROOT_EXPORT_GROUP_ID, "Root"],
      ["namespace:dialogs_zaton", "dialogs_zaton"],
      ["namespace:xr_effects", "xr_effects"],
    ]);
    expect(groups[0]?.declarations.map((declaration) => declaration.name)).toEqual(["start"]);
    expect(groups[1]?.declarations.map((declaration) => declaration.name)).toEqual(["quest.answer"]);
    expect(groups[2]?.declarations.map((declaration) => declaration.name)).toEqual(["run", "stop"]);
  });
});
