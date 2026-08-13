import { beforeEach, describe, expect, it } from "@jest/globals";
import { EventBus, WireEvent } from "@wirestate/core";

import { ArchivesService } from "@/applications/archives/store/archives/archives.service";
import { mockArchiveFileDescriptor } from "@/fixtures/mocks/archive.mocks";
import { setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IInjectedServiceMockDescriptor, mockInjectedService } from "@/fixtures/utils/container";
import { ENotificationSeverity, INotificationPayload, NOTIFICATION_PUSH_EVENT } from "@/lib/notifications";
import { ArchiveFileDescriptor } from "@/lib/xrf/bindings/xray-archive";
import { EArchivesEditorCommand } from "@/lib/xrf/ipc";

const FILE: ArchiveFileDescriptor = mockArchiveFileDescriptor({ name: "textures\\wpn.dds" });

interface IWatchedService {
  service: ArchivesService;
  raised: Array<INotificationPayload>;
}

function watchNotifications(): IWatchedService {
  const { container, service }: IInjectedServiceMockDescriptor<ArchivesService> = mockInjectedService(ArchivesService);
  const raised: Array<INotificationPayload> = [];

  container
    .get(EventBus)
    .subscribe(NOTIFICATION_PUSH_EVENT, (event: WireEvent<INotificationPayload>) =>
      raised.push(event.payload as INotificationPayload)
    );

  return { raised, service };
}

describe("ArchivesService notifications", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("reports a completed extraction, so the outcome survives leaving the editor", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    await service.extractArchiveFile(FILE, "C:\\out\\wpn.dds");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.SUCCESS);
    expect(raised[0].source).toBe("archives");
    expect(raised[0].title).toContain("textures\\wpn.dds");
    expect(raised[0].details).toContain("C:\\out\\wpn.dds");
  });

  it("reports a refused extraction with what the backend said", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE]: () => {
        throw new Error("destination is read only");
      },
    });

    await expect(service.extractArchiveFile(FILE, "C:\\out\\wpn.dds")).rejects.toThrow("read only");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("destination is read only");
  });

  it("counts what a folder extraction wrote", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FOLDER]: () => ({
        destination: "C:\\out",
        extractedCount: 12,
        prefix: "textures",
        size: 2048,
      }),
    });

    await service.extractArchiveFolder("textures", "C:\\out");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.SUCCESS);
    expect(raised[0].title).toContain("12 file(s)");
  });

  it("reports a project that could not be opened", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({
      [EArchivesEditorCommand.OPEN_ARCHIVES_PROJECT]: () => {
        throw new Error("not an archive folder");
      },
    });

    await service.openArchivesProject("C:\\game");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("not an archive folder");
  });
});
