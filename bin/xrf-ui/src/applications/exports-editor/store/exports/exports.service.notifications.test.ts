import { beforeEach, describe, expect, it } from "@jest/globals";
import { EventBus, WireEvent } from "@wirestate/core";

import { ExportsService } from "@/applications/exports-editor/store/exports/exports.service";
import { mockExportsProject } from "@/fixtures/mocks/project.mocks";
import { setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IInjectedServiceMockDescriptor, mockInjectedService } from "@/fixtures/utils/container";
import { EExportsEditorCommand } from "@/lib/ipc";
import { ENotificationSeverity, INotificationPayload, NOTIFICATION_PUSH_EVENT } from "@/lib/notifications";

interface IWatchedService {
  service: ExportsService;
  raised: Array<INotificationPayload>;
}

function watchNotifications(): IWatchedService {
  const { container, service }: IInjectedServiceMockDescriptor<ExportsService> = mockInjectedService(ExportsService);
  const raised: Array<INotificationPayload> = [];

  container
    .get(EventBus)
    .subscribe(NOTIFICATION_PUSH_EVENT, (event: WireEvent<INotificationPayload>) =>
      raised.push(event.payload as INotificationPayload)
    );

  return { raised, service };
}

describe("ExportsService notifications", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("reports a project that could not be parsed", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: () => {
        throw new Error("no scripts directory");
      },
    });

    await service.openExportsProject("C:\\game\\scripts");

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].source).toBe("exports");
    expect(raised[0].details).toContain("no scripts directory");
  });

  it("reports a refresh that could not complete", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({ [EExportsEditorCommand.OPEN_XR_EXPORTS]: mockExportsProject() });

    await service.openExportsProject("C:\\game\\scripts");

    setMockInvokeResponses({
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: () => {
        throw new Error("scripts moved");
      },
    });

    await service.refreshExportsProject();

    expect(raised).toHaveLength(1);
    expect(raised[0].severity).toBe(ENotificationSeverity.ERROR);
    expect(raised[0].details).toContain("scripts moved");
  });

  it("says nothing about a project that opened", async () => {
    const { raised, service }: IWatchedService = watchNotifications();

    setMockInvokeResponses({ [EExportsEditorCommand.OPEN_XR_EXPORTS]: mockExportsProject() });

    await service.openExportsProject("C:\\game\\scripts");

    expect(raised).toHaveLength(0);
  });
});
