import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archive-editor/store/archives/archives.service";
import { mockArchiveFileDescriptor } from "@/fixtures/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/tauri.mocks";
import { IArchiveFileDescriptor } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";

const FILE: IArchiveFileDescriptor = mockArchiveFileDescriptor({ name: "configs\\system.ltx" });

describe("ArchivesService extraction", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("asks the backend for the file by its archived name", async () => {
    const service: ArchivesService = new ArchivesService();

    await service.extractArchiveFile(FILE, "C:\\out\\system.ltx");

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE, {
      name: "configs\\system.ltx",
      destination: "C:\\out\\system.ltx",
    });
    expect(service.singleFileExtraction.value).toBe("C:\\out\\system.ltx");
    expect(service.singleFileExtraction.isLoading).toBe(false);
  });

  it("reports a refused extraction instead of staying loading", async () => {
    const service: ArchivesService = new ArchivesService();

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE]: () => {
        throw new Error("destination is read only");
      },
    });

    await expect(service.extractArchiveFile(FILE, "C:\\out\\system.ltx")).rejects.toThrow("read only");

    // Left loading, the header's extract control stays disabled with no way back.
    expect(service.singleFileExtraction.isLoading).toBe(false);
    expect(String(service.singleFileExtraction.error)).toContain("read only");
  });

  it("clears a reported outcome", async () => {
    const service: ArchivesService = new ArchivesService();

    await service.extractArchiveFile(FILE, "C:\\out\\system.ltx");

    service.clearExtraction();

    expect(service.singleFileExtraction.value).toBeNull();
    expect(service.singleFileExtraction.error).toBeNull();
  });
});
