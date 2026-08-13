import { beforeEach, describe, expect, it } from "@jest/globals";
import { EventBus, WireEvent } from "@wirestate/core";

import { ArchivesService } from "@/applications/archives/services/archives/archives.service";
import { ArchiveFileDescriptor } from "@/core/bindings/xrf-archive";
import { ENotificationSeverity, INotificationPayload, NOTIFICATION_PUSH_EVENT } from "@/core/notifications";
import { mockArchiveFileDescriptor } from "@/fixtures/mocks/archive.mocks";
import { setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IInjectedServiceMockDescriptor, mockInjectedService } from "@/fixtures/utils/container";

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
      ["plugin:archives-editor|extract_archive_file"]: () => {
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
      ["plugin:archives-editor|extract_archive_folder"]: () => ({
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
      ["plugin:archives-editor|open_archives_project"]: () => {
        throw new Error("not an archive folder");
      },
    });

    await service.openArchivesProject("C:\\game");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("not an archive folder");
  });
});
