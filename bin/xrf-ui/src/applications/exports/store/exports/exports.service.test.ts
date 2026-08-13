import { beforeEach, describe, expect, it } from "@jest/globals";

import { ExportsService } from "@/applications/exports/store/exports/exports.service";
import { ExportsProject } from "@/core/bindings/xrf-export";
import { mockExportsProject } from "@/fixtures/mocks/project.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";

const PROJECT: ExportsProject = mockExportsProject();

describe("ExportsService", () => {
  beforeEach(() => {
    setMockInvokeResponses({
      ["plugin:exports-editor|get_xr_exports"]: null,
      ["plugin:exports-editor|open_xr_exports"]: PROJECT,
      ["plugin:exports-editor|close_xr_exports"]: undefined,
    });
  });

  it("restores an existing backend session", async () => {
    setMockInvokeResponses({ ["plugin:exports-editor|get_xr_exports"]: PROJECT });

    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.value).toEqual(PROJECT);
    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:exports-editor|open_xr_exports", expect.anything());
  });

  it("does not open a project when no retained session exists", async () => {
    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.value).toBeNull();
    expect(mockInvoke).toHaveBeenCalledTimes(1);
    expect(mockInvoke).toHaveBeenCalledWith("plugin:exports-editor|get_xr_exports");
  });

  it("recovers from a failed session lookup", async () => {
    setMockInvokeResponses({
      ["plugin:exports-editor|get_xr_exports"]: () => {
        throw new Error("backend unavailable");
      },
    });

    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.project.isLoading).toBe(false);
    expect(service.project.error).toEqual(new Error("backend unavailable"));
  });

  it("opens only the explicitly provided project", async () => {
    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();
    await service.openExportsProject("C:\\chosen\\xrf");

    expect(mockInvoke).toHaveBeenCalledWith("plugin:exports-editor|open_xr_exports", {
      projectPath: "C:\\chosen\\xrf",
    });
    expect(service.project.value).toEqual(PROJECT);
  });

  it("keeps the last successful project when refresh fails", async () => {
    setMockInvokeResponses({
      ["plugin:exports-editor|get_xr_exports"]: PROJECT,
      ["plugin:exports-editor|open_xr_exports"]: () => {
        throw new Error("parse failed");
      },
    });

    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();
    await service.refreshExportsProject();

    expect(service.project.value).toEqual(PROJECT);
    expect(service.project.isLoading).toBe(false);
    expect(service.project.error).toEqual(new Error("parse failed"));
  });

  it("keeps the project and rejects when close fails", async () => {
    setMockInvokeResponses({
      ["plugin:exports-editor|get_xr_exports"]: PROJECT,
      ["plugin:exports-editor|close_xr_exports"]: () => {
        throw new Error("project is busy");
      },
    });

    const service = mockInjectedService(ExportsService).service;

    await service.onProvision();

    await expect(service.closeExportsProject()).rejects.toThrow("project is busy");
    expect(service.project.value).toEqual(PROJECT);
    expect(service.project.isLoading).toBe(false);
  });
});
