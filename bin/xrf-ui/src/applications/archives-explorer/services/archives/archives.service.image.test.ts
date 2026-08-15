import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archives-explorer/services/archives/archives.service";
import { ArchiveFileDescriptor } from "@/core/bindings/xrf-archive";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { createLoadable } from "@/lib/loadable";

const TEXTURE: ArchiveFileDescriptor = mockArchiveFileDescriptor({
  extension: "dds",
  name: "textures\\ui\\wall.dds",
  sizeCompressed: 512,
  sizeReal: 2048,
});

const TEXT: ArchiveFileDescriptor = mockArchiveFileDescriptor({ name: "configs\\system.ltx" });

const PREVIEW = { name: TEXTURE.name, width: 256, height: 256, base64: "iVBORw0KGgo=" };

/**
 * Creates an archive service with fixture files classified by its open project.
 *
 * @returns Service ready to preview the fixture files.
 */
function createService(): ArchivesService {
  const { service } = mockInjectedService(ArchivesService);

  service.project = createLoadable(mockArchivesProject([TEXTURE, TEXT]));

  return service;
}

describe("ArchivesService image preview", () => {
  beforeEach(() => {
    setMockInvokeResponses({ ["plugin:archives|read_image"]: PREVIEW });
  });

  it("decodes a texture instead of reading it as text", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXTURE);

    expect(mockInvoke).toHaveBeenCalledWith("plugin:archives|read_image", { path: TEXTURE.name });
    // The text path would have refused it anyway: this entry is compressed and .dds is not readable.
    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:archives|read_file", expect.anything());
    expect(service.content.value?.kind === "image" ? service.content.value.preview.width : null).toBe(256);
  });

  it("leaves text files on the text path", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXT);

    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:archives|read_image", expect.anything());
    expect(service.content.value?.kind === "image" ? service.content.value.preview : null).toBeNull();
  });

  it("reports a failed decode instead of staying loading", async () => {
    const service: ArchivesService = createService();

    setMockInvokeResponses({
      ["plugin:archives|read_image"]: () => {
        throw new Error("unsupported DXT format");
      },
    });

    await service.selectArchiveFile(TEXTURE);

    expect(service.content.isLoading).toBe(false);
    expect(String(service.content.error)).toContain("unsupported DXT format");
  });

  it("retries the decode rather than falling back to a text read", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXTURE);
    await service.retrySelectedFile();

    const imageCalls = mockInvoke.mock.calls.filter(([command]) => command === "plugin:archives|read_image");

    expect(imageCalls).toHaveLength(2);
    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:archives|read_file", expect.anything());
  });

  it("drops the decoded image when the selection changes", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXTURE);
    expect(service.content.value?.kind === "image" ? service.content.value.preview : null).not.toBeNull();

    // An image outliving its file would be shown beside the next selection.
    service.selectArchiveDirectory("textures");
    expect(service.content.value?.kind === "image" ? service.content.value.preview : null).toBeNull();
  });
});
