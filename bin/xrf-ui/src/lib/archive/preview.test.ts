import { describe, expect, it } from "@jest/globals";

import { mockArchiveFileDescriptor } from "@/fixtures/archive.mocks";
import { getArchivePreviewSupport } from "@/lib/archive/preview";

describe("archive preview support", () => {
  it("accepts uncompressed LTX and script files within the backend limit", () => {
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor())).toEqual({ kind: "supported" });
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "script", name: "actor.SCRIPT" }))).toEqual({
      kind: "supported",
    });
  });

  it("identifies each unsupported reason before a backend read", () => {
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" }))
    ).toEqual({
      kind: "unsupported-extension",
      extension: "dds",
    });
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor({ sizeReal: 2048, sizeCompressed: 1024 }))).toEqual({
      kind: "compressed",
    });
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor({ sizeReal: 10 * 1024 * 1024 + 1 }))).toEqual({
      kind: "too-large",
      maximumSize: 10 * 1024 * 1024,
    });
  });

  it("uses the extension supplied by the archive descriptor", () => {
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "script", name: "actor.bin" }))).toEqual({
      kind: "supported",
    });
  });
});
