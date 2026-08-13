import { describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archives/store/archives";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { createLoadable } from "@/lib/loadable";
import { ArchiveFileDescriptor, ProjectReadResult } from "@/lib/xrf/bindings/xrf-archive";
import { EArchivesEditorCommand } from "@/lib/xrf/ipc";

function ignoreReadResult(): void {}

function mockArchivesService(files: Array<ArchiveFileDescriptor>): ArchivesService {
  const { service } = mockInjectedService(ArchivesService);

  service.project = createLoadable(mockArchivesProject(files));

  return service;
}

describe("ArchivesService file selection", () => {
  it("loads supported selected files", async () => {
    const descriptor = mockArchiveFileDescriptor();
    const result: ProjectReadResult = { name: descriptor.name, content: "[system]", size: 8 };

    setMockInvokeResponses({ [EArchivesEditorCommand.READ_ARCHIVE_FILE]: result });

    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);

    expect(service.selectedFile).toStrictEqual(descriptor);
    expect(service.content.value?.kind === "text" ? service.content.value.result : null).toEqual(result);
    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, { path: descriptor.name });
  });

  it("selects unsupported files without invoking the read command", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "ogf", name: "meshes\\actor.ogf" });
    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);

    expect(service.selectedFile).toStrictEqual(descriptor);
    expect(service.content.value?.kind === "text" ? service.content.value.result : null).toBeNull();
    expect(mockInvoke).not.toHaveBeenCalled();
  });

  it("allows only the latest selection to publish a completed read", async () => {
    const first = mockArchiveFileDescriptor({ name: "configs\\first.ltx" });
    const second = mockArchiveFileDescriptor({ name: "configs\\second.ltx" });
    let resolveFirst: (value: ProjectReadResult) => void = ignoreReadResult;
    let resolveSecond: (value: ProjectReadResult) => void = ignoreReadResult;
    const firstResult: Promise<ProjectReadResult> = new Promise((resolve) => {
      resolveFirst = resolve;
    });
    const secondResult: Promise<ProjectReadResult> = new Promise((resolve) => {
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

    expect(service.selectedFile).toStrictEqual(second);
    expect(service.content.value?.kind === "text" ? service.content.value.result.name : null).toBe(second.name);
    expect(service.content.value?.kind === "text" ? service.content.value.result.content : null).toBe("second");
  });

  it("clears file state when the project closes", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });

    setMockInvokeResponses({ [EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT]: undefined });

    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);
    await service.closeArchivesProject();

    expect(service.selectedFile).toBeNull();
    expect(service.content.value?.kind === "text" ? service.content.value.result : null).toBeNull();
  });

  it("clears file state when the project is reset", async () => {
    const descriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });
    const service: ArchivesService = mockArchivesService([descriptor]);

    await service.selectArchiveFile(descriptor);
    service.resetArchivesProject();

    expect(service.selectedFile).toBeNull();
    expect(service.content.value?.kind === "text" ? service.content.value.result : null).toBeNull();
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
    expect(service.selectedFile).toStrictEqual(descriptor);
  });
});
