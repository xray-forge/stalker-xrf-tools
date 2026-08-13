import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archives/store/archives/archives.service";
import { Nullable } from "@/core/types/general";
import { mockArchiveFileDescriptor } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { ArchiveFileDescriptor } from "@/lib/xrf/bindings/xrf-archive";
import { EArchivesEditorCommand } from "@/lib/xrf/ipc";

const FILE: ArchiveFileDescriptor = mockArchiveFileDescriptor({ name: "configs\\system.ltx" });

/** The operation union carries every kind of write, so a file assertion has to name its own. */
function extractedFile(service: ArchivesService): Nullable<string> {
  return service.operation.value?.kind === "extract-file" ? service.operation.value.destination : null;
}

describe("ArchivesService extraction", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("asks the backend for the file by its archived name", async () => {
    const { service } = mockInjectedService(ArchivesService);

    await service.extractArchiveFile(FILE, "C:\\out\\system.ltx");

    expect(mockInvoke).toHaveBeenCalledWith(EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE, {
      name: "configs\\system.ltx",
      destination: "C:\\out\\system.ltx",
    });
    expect(extractedFile(service)).toBe("C:\\out\\system.ltx");
    expect(service.operation.isLoading).toBe(false);
  });

  it("reports a refused extraction instead of staying loading", async () => {
    const { service } = mockInjectedService(ArchivesService);

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE]: () => {
        throw new Error("destination is read only");
      },
    });

    await expect(service.extractArchiveFile(FILE, "C:\\out\\system.ltx")).rejects.toThrow("read only");

    // Left loading, the header's extract control stays disabled with no way back.
    expect(service.operation.isLoading).toBe(false);
    expect(String(service.operation.error)).toContain("read only");
  });

  it("clears a reported outcome", async () => {
    const { service } = mockInjectedService(ArchivesService);

    await service.extractArchiveFile(FILE, "C:\\out\\system.ltx");

    service.clearOperation();

    expect(service.operation.value?.kind === "extract-file" ? service.operation.value.destination : null).toBeNull();
    expect(service.operation.error).toBeNull();
  });
});
