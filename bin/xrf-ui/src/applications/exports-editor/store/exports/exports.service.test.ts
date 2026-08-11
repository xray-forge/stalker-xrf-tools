import { beforeEach, describe, expect, it } from "@jest/globals";

import { ExportsService } from "@/applications/exports-editor/store/exports/exports.service";
import { mockExportsProject } from "@/fixtures/project.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/tauri.mocks";
import { IExportsProject } from "@/lib/exports";
import { EExportsEditorCommand } from "@/lib/ipc";

const PROJECT: IExportsProject = mockExportsProject();

describe("ExportsService", () => {
  beforeEach(() => {
    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: null,
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: PROJECT,
      [EExportsEditorCommand.CLOSE_XR_EXPORTS]: undefined,
    });
  });

  it("restores an existing backend session", async () => {
    setMockInvokeResponses({ [EExportsEditorCommand.GET_XR_EXPORTS]: PROJECT });

    const service = new ExportsService();

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.value).toEqual(PROJECT);
    expect(mockInvoke).not.toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, expect.anything());
  });

  it("does not open a project when no retained session exists", async () => {
    const service = new ExportsService();

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.value).toBeNull();
    expect(mockInvoke).toHaveBeenCalledTimes(1);
    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.GET_XR_EXPORTS);
  });

  it("recovers from a failed session lookup", async () => {
    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: () => {
        throw new Error("backend unavailable");
      },
    });

    const service = new ExportsService();

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.isLoading).toBe(false);
    expect(service.project.error).toEqual(new Error("backend unavailable"));
  });

  it("opens only the explicitly provided project", async () => {
    const service = new ExportsService();

    await service.onProvision();
    await service.openExportsProject("C:\\chosen\\xrf");

    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, {
      projectPath: "C:\\chosen\\xrf",
    });
    expect(service.project.value).toEqual(PROJECT);
  });

  it("keeps the last successful project when refresh fails", async () => {
    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: PROJECT,
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: () => {
        throw new Error("parse failed");
      },
    });

    const service = new ExportsService();

    await service.onProvision();
    await service.refreshExportsProject();

    expect(service.project.value).toEqual(PROJECT);
    expect(service.project.isLoading).toBe(false);
    expect(service.project.error).toEqual(new Error("parse failed"));
  });

  it("keeps the project and rejects when close fails", async () => {
    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: PROJECT,
      [EExportsEditorCommand.CLOSE_XR_EXPORTS]: () => {
        throw new Error("project is busy");
      },
    });

    const service = new ExportsService();

    await service.onProvision();

    await expect(service.closeExportsProject()).rejects.toThrow("project is busy");
    expect(service.project.value).toEqual(PROJECT);
    expect(service.project.isLoading).toBe(false);
  });
});
