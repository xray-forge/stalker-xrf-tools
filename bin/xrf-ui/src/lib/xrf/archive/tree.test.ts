import { describe, expect, it } from "@jest/globals";

import { mockArchiveFileDescriptor } from "@/fixtures/mocks/archive.mocks";
import { IArchiveTreeItem, isUnderArchiveDirectory, parseTree } from "@/lib/xrf/archive/tree";

const CONFIGS_DIALOGS: string = ["configs", "gameplay", "dialogs.xml"].join("\\");
const CONFIGS_BACKUP: string = ["configs_backup", "a.ltx"].join("\\");
const CONFIGS_DIRECTORY: string = ["configs", "gameplay", ""].join("\\");

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

describe("isUnderArchiveDirectory", () => {
  function under(name: string, prefix: string, sizeReal: number = 128): boolean {
    return isUnderArchiveDirectory(mockArchiveFileDescriptor({ name, sizeReal }), prefix);
  }

  it("matches whole path segments only", () => {
    expect(under(CONFIGS_DIALOGS, "configs")).toBe(true);
    // A plain startsWith would pull the backup folder into an extraction of `configs`.
    expect(under(CONFIGS_BACKUP, "configs")).toBe(false);
  });

  it("takes everything for an empty prefix", () => {
    expect(under(CONFIGS_DIALOGS, "")).toBe(true);
  });

  it("ignores case and a trailing separator on the prefix", () => {
    expect(under(CONFIGS_DIALOGS, "CONFIGS")).toBe(true);
    expect(under(CONFIGS_DIALOGS, "configs\\")).toBe(true);
  });

  it("excludes what the backend will not write", () => {
    // Counting these would promise more files than extraction delivers.
    expect(under(CONFIGS_DIALOGS, "configs", 0)).toBe(false);
    expect(under(CONFIGS_DIRECTORY, "configs")).toBe(false);
  });
});
