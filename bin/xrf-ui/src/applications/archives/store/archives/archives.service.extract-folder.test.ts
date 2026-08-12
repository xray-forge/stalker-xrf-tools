import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archives/store/archives/archives.service";
import { Nullable } from "@/core/types/general";
import { mockArchiveFileDescriptor } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { ArchiveExtractFolderResult } from "@/lib/bindings/xray-archive";
import { EArchivesEditorCommand } from "@/lib/ipc";

/** The operation union carries every kind of write, so a folder assertion has to name its own. */
function extractedFolder(service: ArchivesService): Nullable<ArchiveExtractFolderResult> {
  return service.operation.value?.kind === "extract-folder" ? service.operation.value.result : null;
}

describe("ArchivesService folder extraction", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("sends the directory prefix and destination root", async () => {
    const { service } = mockInjectedService(ArchivesService);

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER]: {
        prefix: "configs",
        destination: "C:\\out",
        extractedCount: 12,
        size: 4096,
      },
    });

    await service.extractArchiveFolder("configs", "C:\\out");

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER, {
      prefix: "configs",
      destination: "C:\\out",
    });
    expect(extractedFolder(service)?.extractedCount).toBe(12);
  });

  it("treats the archive root as an empty prefix", async () => {
    const { service } = mockInjectedService(ArchivesService);

    service.selectArchiveDirectory("");

    expect(service.selectedDirectory).toBe("");

    await service.extractArchiveFolder("", "C:\\out");

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER, {
      prefix: "",
      destination: "C:\\out",
    });
  });

  it("reports a refused extraction instead of staying loading", async () => {
    const { service } = mockInjectedService(ArchivesService);

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER]: () => {
        throw new Error("destination is read only");
      },
    });

    await expect(service.extractArchiveFolder("configs", "C:\\out")).rejects.toThrow("read only");

    expect(service.operation.isLoading).toBe(false);
    expect(String(service.operation.error)).toContain("read only");
  });

  it("keeps file and directory selection mutually exclusive", async () => {
    const { service } = mockInjectedService(ArchivesService);

    service.selectArchiveDirectory("configs");
    expect(service.selectedDirectory).toBe("configs");

    // Both being set at once would leave the content area with two things claiming to be selected.
    await service.selectArchiveFile(mockArchiveFileDescriptor({ name: "configs\\system.ltx" }));
    expect(service.selectedDirectory).toBeNull();
    expect(service.selectedFile).not.toBeNull();

    service.selectArchiveDirectory("configs");
    expect(service.selectedFile).toBeNull();
  });
});
