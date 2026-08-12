import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archive-editor/store/archives/archives.service";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { IArchiveFileDescriptor } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { createLoadable } from "@/lib/loadable";

const TEXTURE: IArchiveFileDescriptor = mockArchiveFileDescriptor({
  extension: "dds",
  name: "textures\\ui\\wall.dds",
  sizeCompressed: 512,
  sizeReal: 2048,
});

const TEXT: IArchiveFileDescriptor = mockArchiveFileDescriptor({ name: "configs\\system.ltx" });

const PREVIEW = { name: TEXTURE.name, width: 256, height: 256, base64: "iVBORw0KGgo=" };

/**
 * The service reads the image rules off the open project rather than from its own constants, so a
 * project has to be present for anything to be classified as a texture.
 */
function createService(): ArchivesService {
  const { service } = mockInjectedService(ArchivesService);

  service.project = createLoadable(mockArchivesProject([TEXTURE, TEXT]));

  return service;
}

describe("ArchivesService image preview", () => {
  beforeEach(() => {
    setMockInvokeResponses({ [EArchivesEditorCommand.READ_ARCHIVE_IMAGE]: PREVIEW });
  });

  it("decodes a texture instead of reading it as text", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXTURE);

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_IMAGE, { path: TEXTURE.name });
    // The text path would have refused it anyway: this entry is compressed and .dds is not readable.
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, expect.anything());
    expect(service.content.value?.kind === "image" ? service.content.value.preview.width : null).toBe(256);
  });

  it("leaves text files on the text path", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXT);

    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_IMAGE, expect.anything());
    expect(service.content.value?.kind === "image" ? service.content.value.preview : null).toBeNull();
  });

  it("reports a failed decode instead of staying loading", async () => {
    const service: ArchivesService = createService();

    setMockInvokeResponses({
      [EArchivesEditorCommand.READ_ARCHIVE_IMAGE]: () => {
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

    const imageCalls = mockInvoke.mock.calls.filter(
      ([command]) => command === EArchivesEditorCommand.READ_ARCHIVE_IMAGE
    );

    expect(imageCalls).toHaveLength(2);
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, expect.anything());
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
