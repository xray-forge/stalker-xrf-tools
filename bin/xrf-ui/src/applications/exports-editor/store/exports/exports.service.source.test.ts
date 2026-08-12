import { beforeEach, describe, expect, it } from "@jest/globals";

import { ExportsService } from "@/applications/exports-editor/store/exports/exports.service";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { IExportSourceContent } from "@/lib/exports";
import { EExportsEditorCommand } from "@/lib/ipc";

const SOURCE: IExportSourceContent = {
  name: "xr_effects.play",
  path: "effects/sound.ts",
  line: 18,
  endLine: 21,
  content: 'extern("xr_effects.play", (): void => {});',
};

describe("ExportsService export source", () => {
  beforeEach(() => {
    setMockInvokeResponses({ [EExportsEditorCommand.GET_XR_EXPORT_SOURCE]: SOURCE });
  });

  it("reads the source of one declaration by name", async () => {
    const service: ExportsService = new ExportsService();

    await expect(service.readExportSource("xr_effects.play")).resolves.toEqual(SOURCE);
    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.GET_XR_EXPORT_SOURCE, { name: "xr_effects.play" });
  });

  it("propagates a failed read to its caller", async () => {
    // Reporting is the view's job here, so the service must not swallow this into a null result.
    const service: ExportsService = new ExportsService();

    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORT_SOURCE]: () => {
        throw new Error("declaration file is gone");
      },
    });

    await expect(service.readExportSource("xr_effects.play")).rejects.toThrow("declaration file is gone");
  });

  it("holds no source state of its own", () => {
    // The body belongs to whatever is on screen; keeping it here would make the service arbitrate
    // between in-flight reads that the viewing effect already knows how to abandon.
    expect(new ExportsService()).not.toHaveProperty("source");
  });
});
