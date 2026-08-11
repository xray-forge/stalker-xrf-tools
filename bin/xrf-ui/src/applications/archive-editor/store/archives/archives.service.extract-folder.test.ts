import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archive-editor/store/archives/archives.service";
import { mockArchiveFileDescriptor } from "@/fixtures/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/tauri.mocks";
import { EArchivesEditorCommand } from "@/lib/ipc";

describe("ArchivesService folder extraction", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("sends the directory prefix and destination root", async () => {
    const service: ArchivesService = new ArchivesService();

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
    expect(service.folderExtraction.value?.extractedCount).toBe(12);
  });

  it("treats the archive root as an empty prefix", async () => {
    const service: ArchivesService = new ArchivesService();

    service.selectArchiveDirectory("");

    expect(service.directoryPath).toBe("");

    await service.extractArchiveFolder("", "C:\\out");

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER, {
      prefix: "",
      destination: "C:\\out",
    });
  });

  it("reports a refused extraction instead of staying loading", async () => {
    const service: ArchivesService = new ArchivesService();

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER]: () => {
        throw new Error("destination is read only");
      },
    });

    await expect(service.extractArchiveFolder("configs", "C:\\out")).rejects.toThrow("read only");

    expect(service.folderExtraction.isLoading).toBe(false);
    expect(String(service.folderExtraction.error)).toContain("read only");
  });

  it("keeps file and directory selection mutually exclusive", async () => {
    const service: ArchivesService = new ArchivesService();

    service.selectArchiveDirectory("configs");
    expect(service.directoryPath).toBe("configs");

    // Both being set at once would leave the content area with two things claiming to be selected.
    await service.selectArchiveFile(mockArchiveFileDescriptor({ name: "configs\\system.ltx" }));
    expect(service.directoryPath).toBeNull();
    expect(service.fileDescriptor).not.toBeNull();

    service.selectArchiveDirectory("configs");
    expect(service.fileDescriptor).toBeNull();
  });
});
