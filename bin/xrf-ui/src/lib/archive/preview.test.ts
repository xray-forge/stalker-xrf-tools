import { describe, expect, it } from "@jest/globals";

import { mockArchiveFileDescriptor, mockArchiveReadPolicy } from "@/fixtures/mocks/archive.mocks";
import { getArchivePreviewSupport } from "@/lib/archive/preview";
import { IArchiveReadPolicy } from "@/lib/archive/types";

const READ_POLICY: IArchiveReadPolicy = mockArchiveReadPolicy();

describe("archive preview support", () => {
  it.each(READ_POLICY.extensions)("accepts uncompressed .%s files within the backend limit", (extension: string) => {
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ extension, name: `preview.${extension}` }), READ_POLICY)
    ).toEqual({ kind: "supported" });
  });

  it("accepts the normalized extension regardless of filename casing", () => {
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "script", name: "actor.SCRIPT" }), READ_POLICY)
    ).toEqual({ kind: "supported" });
  });

  it("identifies each unsupported reason before a backend read", () => {
    // `.dds` is decoded as an image now, so an unreadable-extension case needs a different type.
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "ogf", name: "meshes\\actor.ogf" }), READ_POLICY)
    ).toEqual({
      kind: "unsupported-extension",
      extension: "ogf",
    });
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ sizeReal: 2048, sizeCompressed: 1024 }), READ_POLICY)
    ).toEqual({ kind: "compressed" });
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ sizeReal: READ_POLICY.maximumSize + 1 }), READ_POLICY)
    ).toEqual({ kind: "too-large", maximumSize: READ_POLICY.maximumSize });
  });

  it("routes textures to the image path, compressed or not", () => {
    // Compression is invisible by the time there is an image to decode, so unlike text it is no reason
    // to refuse. The size limit is the decoder's own, not the text read policy's.
    expect(
      getArchivePreviewSupport(
        mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds", sizeReal: 2048, sizeCompressed: 512 }),
        READ_POLICY
      )
    ).toEqual({ kind: "image" });

    expect(
      getArchivePreviewSupport(
        mockArchiveFileDescriptor({ extension: "dds", sizeReal: READ_POLICY.maximumImageSize + 1 }),
        READ_POLICY
      )
    ).toEqual({ kind: "too-large", maximumSize: READ_POLICY.maximumImageSize });
  });

  it("uses the extension supplied by the archive descriptor", () => {
    expect(
      getArchivePreviewSupport(mockArchiveFileDescriptor({ extension: "script", name: "actor.bin" }), READ_POLICY)
    ).toEqual({ kind: "supported" });
  });

  it("uses backend-provided policy values", () => {
    const policy: IArchiveReadPolicy = mockArchiveReadPolicy({
      extensions: ["xml"],
      maximumSize: 1024,
      supportsCompressedFiles: true,
    });

    expect(
      getArchivePreviewSupport(
        mockArchiveFileDescriptor({ extension: "xml", name: "preview.xml", sizeReal: 1024, sizeCompressed: 512 }),
        policy
      )
    ).toEqual({ kind: "supported" });
    expect(getArchivePreviewSupport(mockArchiveFileDescriptor(), policy)).toEqual({
      kind: "unsupported-extension",
      extension: "ltx",
    });
  });
});
