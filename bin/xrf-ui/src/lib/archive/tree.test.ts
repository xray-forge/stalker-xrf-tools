import { describe, expect, it } from "@jest/globals";

import { mockArchiveFileDescriptor } from "@/fixtures/archive.mocks";
import { IArchiveTreeItem, parseTree } from "@/lib/archive/tree";

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

});
