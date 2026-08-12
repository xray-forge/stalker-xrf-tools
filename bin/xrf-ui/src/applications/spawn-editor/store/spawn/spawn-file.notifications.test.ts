import { beforeEach, describe, expect, it } from "@jest/globals";
import { EventBus, WireEvent } from "@wirestate/core";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn/spawn-file.service";
import { setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IInjectedServiceMockDescriptor, mockInjectedService } from "@/fixtures/utils/container";
import { ESpawnsEditorCommand } from "@/lib/ipc";
import { createLoadable } from "@/lib/loadable";
import { ENotificationSeverity, INotificationPayload, NOTIFICATION_PUSH_EVENT } from "@/lib/notifications";
import { ISpawnFile } from "@/lib/spawn-file";

interface IWatchedService {
  service: SpawnFileService;
  raised: Array<INotificationPayload>;
}

function watchNotifications(): IWatchedService {
  const { container, service }: IInjectedServiceMockDescriptor<SpawnFileService> =
    mockInjectedService(SpawnFileService);
  const raised: Array<INotificationPayload> = [];

  container
    .get(EventBus)
    .subscribe(NOTIFICATION_PUSH_EVENT, (event: WireEvent<INotificationPayload>) =>
      raised.push(event.payload as INotificationPayload)
    );

  return { raised, service };
}

/** Saving and exporting assert an open file, so one has to be present before either is reachable. */
function withOpenSpawnFile(service: SpawnFileService): void {
  service.spawnFile = createLoadable({} as ISpawnFile);
}

describe("SpawnFileService notifications", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("reports a written save", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    withOpenSpawnFile(service);

    await service.saveSpawnFile("C:\\out\\all.spawn");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.SUCCESS);
    expect(raised[0].source).toBe("spawns");
    expect(raised[0].details).toContain("C:\\out\\all.spawn");
  });

  it("reports a save that failed, which the state alone cannot say", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    withOpenSpawnFile(service);

    setMockInvokeResponses({
      [ESpawnsEditorCommand.SAVE_SPAWN_FILE]: () => {
        throw new Error("destination is read only");
      },
    });

    await service.saveSpawnFile("C:\\out\\all.spawn");

    // The service lands back on ready either way, so the notification is the only difference between
    // a save that wrote and one that did not.
    expect(service.spawnFile.isLoading).toBe(false);
    expect(service.spawnFile.error).toBeNull();
    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("destination is read only");
  });

  it("reports an export that failed", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    withOpenSpawnFile(service);

    setMockInvokeResponses({
      [ESpawnsEditorCommand.EXPORT_SPAWN_FILE]: () => {
        throw new Error("no such directory");
      },
    });

    await service.exportSpawnFile("C:\\out\\unpacked");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("no such directory");
  });

  it("reports an import that failed", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({
      [ESpawnsEditorCommand.IMPORT_SPAWN_FILE]: () => {
        throw new Error("not a spawn directory");
      },
    });

    await service.importSpawnFile("C:\\game\\unpacked");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("not a spawn directory");
  });

  it("says nothing about an open that worked", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    await service.openSpawnFile("C:\\game\\all.spawn");

    // Loading something is its own confirmation - the editor fills. Only writes and failures are worth
    // a record that outlives the screen.
    expect(raised).toHaveLength(0);
  });
});
