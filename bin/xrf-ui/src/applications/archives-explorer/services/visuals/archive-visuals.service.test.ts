import { beforeEach, describe, expect, it } from "@jest/globals";
import { isObservableProp } from "@wirestate/mobx";

import { ArchiveVisualsService } from "@/applications/archives-explorer/services/visuals";
import { SelectedVisualDescription } from "@/core/bindings/types/xrf-app";
import { resetMockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import {
  mockPackedSubmesh,
  mockSelectedVisual,
  MockVisualBuffer,
  mockVisualDescription,
} from "@/fixtures/mocks/visual.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { Nullable } from "@/lib/types/general";

const ROOT: string = "C:\\game\\db";
const ENTRY: string = "meshes\\actors\\stalker.ogf";

/** A previewable model whose description matches the buffer returned beside it. */
function mockPreviewable(): { selected: SelectedVisualDescription; buffer: ArrayBuffer } {
  const buffer: MockVisualBuffer = new MockVisualBuffer();
  const submesh = mockPackedSubmesh(buffer);

  return {
    selected: mockSelectedVisual({
      source: { kind: "asset", logicalPath: ENTRY },
      world: { asset: null, roots: [ROOT] },
      description: mockVisualDescription({ submeshes: [submesh], bufferLength: buffer.byteLength }),
    }),
    buffer: buffer.toArrayBuffer(),
  };
}

describe("ArchiveVisualsService", () => {
  beforeEach(() => {
    resetMockInvoke();
  });

  it("applies its mobx annotations", () => {
    const { service } = mockInjectedService(ArchiveVisualsService);

    expect(isObservableProp(service, "visual")).toBe(true);
    expect(isObservableProp(service, "textures")).toBe(true);
  });

  it("previews an archived entry by its engine identity, in the archive's own world", async () => {
    // The entry is addressed logically rather than by a filesystem path, because it has none: the bytes are inside a
    // volume, and the world the archive was opened at is what makes the name resolvable.
    const { selected, buffer } = mockPreviewable();
    const { service } = mockInjectedService(ArchiveVisualsService);

    let openParameters: Nullable<Record<string, unknown>> = null;

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: (parameters?: Record<string, unknown>) => {
        openParameters = parameters ?? null;

        return selected;
      },
      ["plugin:visuals|read_geometry"]: buffer,
    });

    await service.preview(ROOT, ENTRY);

    expect(openParameters).toEqual({
      source: { kind: "asset", logicalPath: ENTRY },
      world: { asset: null, roots: [ROOT] },
    });
    expect(service.visual.value?.views.submeshes).toHaveLength(1);
    expect(service.visual.error).toBeNull();
  });

  it("reports a model it could not read rather than leaving the viewport unexplained", async () => {
    const { service } = mockInjectedService(ArchiveVisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: () => {
        throw new Error("chunk declares more bytes than the entry holds");
      },
    });

    await service.preview(ROOT, ENTRY);

    expect(service.visual.value).toBeNull();
    expect(service.visual.error?.message).toBe("chunk declares more bytes than the entry holds");
    expect(service.visual.isLoading).toBe(false);
  });

  it("drops what it previewed when the selection stops being a model", async () => {
    const { selected, buffer } = mockPreviewable();
    const { service } = mockInjectedService(ArchiveVisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: selected,
      ["plugin:visuals|read_geometry"]: buffer,
    });

    await service.preview(ROOT, ENTRY);
    service.clear();

    expect(service.visual.value).toBeNull();
    expect(service.textures.size).toBe(0);
  });
});
