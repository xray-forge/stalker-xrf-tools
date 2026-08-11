import { describe, expect, it } from "@jest/globals";

import { mockArchiveFileDescriptor } from "@/fixtures/archive.mocks";
import { filterArchiveTree, IArchiveTreeItem, parseTree } from "@/lib/archive/tree";

describe("archive tree", () => {
  it("builds a directory-first tree with descriptors on file leaves", () => {
    const config = mockArchiveFileDescriptor({ name: "configs\\z.ltx" });
    const script = mockArchiveFileDescriptor({ name: "scripts\\actor.script" });
    const root = mockArchiveFileDescriptor({ name: "readme.ltx" });
    const items: Array<IArchiveTreeItem> = parseTree([root, script, config], "\\");

    expect(items.map((item) => item.label)).toEqual(["configs", "scripts", "readme.ltx"]);
    expect(items[0]).toMatchObject({ id: "directory:configs", kind: "directory" });

    const configs = items[0];

    expect(configs.kind).toBe("directory");

    if (configs.kind === "directory") {
      expect(configs.children[0]).toMatchObject({
        id: "file:configs\\z.ltx",
        kind: "file",
        descriptor: config,
      });
    }
  });

  it("retains matching ancestors and expands the path to a file match", () => {
    const items: Array<IArchiveTreeItem> = parseTree(
      [
        mockArchiveFileDescriptor({ name: "configs\\weapons\\rifles.ltx" }),
        mockArchiveFileDescriptor({ name: "scripts\\actor.script" }),
      ],
      "\\"
    );
    const filtered = filterArchiveTree(items, "rifles");

    expect(filtered.items).toHaveLength(1);
    expect(filtered.items[0].label).toBe("configs");
    expect(filtered.expandedItems).toEqual(expect.arrayContaining(["directory:configs", "directory:configs\\weapons"]));
  });

  it("keeps a matching directory's descendants", () => {
    const items: Array<IArchiveTreeItem> = parseTree(
      [
        mockArchiveFileDescriptor({ name: "configs\\weapons\\rifles.ltx" }),
        mockArchiveFileDescriptor({ name: "configs\\weapons\\pistols.ltx" }),
      ],
      "\\"
    );
    const filtered = filterArchiveTree(items, "weapons");
    const configs = filtered.items[0];

    expect(configs.kind).toBe("directory");

    if (configs.kind === "directory") {
      const weapons = configs.children[0];

      expect(weapons.kind).toBe("directory");

      if (weapons.kind === "directory") {
        expect(weapons.children.map((item) => item.label)).toEqual(["pistols.ltx", "rifles.ltx"]);
      }
    }
  });

  it("returns an empty tree when nothing matches", () => {
    const items: Array<IArchiveTreeItem> = parseTree([mockArchiveFileDescriptor()], "\\");

    expect(filterArchiveTree(items, "missing").items).toEqual([]);
  });
});
