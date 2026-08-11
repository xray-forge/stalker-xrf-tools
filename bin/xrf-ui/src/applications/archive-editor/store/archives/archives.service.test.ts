import { describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IArchiveFileDescriptor, IArchiveFileReadResult } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";
import { createLoadable } from "@/lib/loadable";

function ignoreReadResult(): void {}

function mockArchivesService(files: Array<IArchiveFileDescriptor>): ArchivesService {
  const service: ArchivesService = new ArchivesService();

  service.project = createLoadable(mockArchivesProject(files));

  return service;
}

describe("ArchivesService file selection", () => {
  it("loads supported selected files", async () => {
    const descriptor = mockArchiveFileDescriptor();
    const result: IArchiveFileReadResult = { name: descriptor.name, content: "[system]", size: 8 };

    setMockInvokeResponses({ [EArchivesEditorCommand.READ_ARCHIVE_FILE]: result });

    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);

    expect(service.fileDescriptor).toStrictEqual(descriptor);
    expect(service.file.value).toEqual(result);
    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, { path: descriptor.name });
  });

  it("selects unsupported files without invoking the read command", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "ogf", name: "meshes\\actor.ogf" });
    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);

    expect(service.fileDescriptor).toStrictEqual(descriptor);
    expect(service.file.value).toBeNull();
    expect(mockInvoke).not.toHaveBeenCalled();
  });

  it("allows only the latest selection to publish a completed read", async () => {
    const first = mockArchiveFileDescriptor({ name: "configs\\first.ltx" });
    const second = mockArchiveFileDescriptor({ name: "configs\\second.ltx" });
    let resolveFirst: (value: IArchiveFileReadResult) => void = ignoreReadResult;
    let resolveSecond: (value: IArchiveFileReadResult) => void = ignoreReadResult;
    const firstResult: Promise<IArchiveFileReadResult> = new Promise((resolve) => {
      resolveFirst = resolve;
    });
    const secondResult: Promise<IArchiveFileReadResult> = new Promise((resolve) => {
      resolveSecond = resolve;
    });

    setMockInvokeResponses({
      [EArchivesEditorCommand.READ_ARCHIVE_FILE]: (args?: Record<string, unknown>) =>
        args?.path === first.name ? firstResult : secondResult,
    });

    const service: ArchivesService = mockArchivesService([first, second]);
    const firstRead: Promise<void> = service.selectArchiveFile(first);
    const secondRead: Promise<void> = service.selectArchiveFile(second);

    resolveSecond({ name: second.name, content: "second", size: 6 });
    await secondRead;
    resolveFirst({ name: first.name, content: "first", size: 5 });
    await firstRead;

    expect(service.fileDescriptor).toStrictEqual(second);
    expect(service.file.value?.name).toBe(second.name);
    expect(service.file.value?.content).toBe("second");
  });

  it("clears file state when the project closes", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });

    setMockInvokeResponses({ [EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT]: undefined });

    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);
    await service.closeArchivesProject();

    expect(service.fileDescriptor).toBeNull();
    expect(service.file.value).toBeNull();
  });

  it("clears file state when the project is reset", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });
    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);
    service.resetArchivesProject();

    expect(service.fileDescriptor).toBeNull();
    expect(service.file.value).toBeNull();
  });

  it("preserves the open project and selection when closing fails", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });

    setMockInvokeResponses({
      [EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT]: () => {
        throw new Error("archive is busy");
      },
    });

    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);

    await expect(service.closeArchivesProject()).rejects.toThrow("archive is busy");
    expect(service.fileDescriptor).toStrictEqual(descriptor);
  });
});
