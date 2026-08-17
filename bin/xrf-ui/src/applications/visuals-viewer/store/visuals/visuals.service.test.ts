import { beforeEach, describe, expect, it } from "@jest/globals";
import { isComputedProp, isObservableProp } from "@wirestate/mobx";

import { VisualsService } from "@/applications/visuals-viewer/store/visuals";
import { SelectedVisualDescription } from "@/core/bindings/xrf-app-visuals";
import { resetMockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import {
  mockPackedSubmesh,
  mockSelectedVisual,
  MockVisualBuffer,
  mockVisualDescription,
} from "@/fixtures/mocks/visual.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { Nullable } from "@/lib/types/general";

/** A selected visual whose description matches the buffer returned beside it. */
function mockOpenableVisual(path: string = "C:\\gamedata\\wpn_ak74.ogf"): {
  selected: SelectedVisualDescription;
  buffer: ArrayBuffer;
} {
  const buffer: MockVisualBuffer = new MockVisualBuffer();
  const submesh = mockPackedSubmesh(buffer);

  return {
    selected: mockSelectedVisual({
      source: { kind: "file", path },
      description: mockVisualDescription({ submeshes: [submesh], bufferLength: buffer.byteLength }),
    }),
    buffer: buffer.toArrayBuffer(),
  };
}

describe("VisualsService observability", () => {
  it("applies its mobx annotations", () => {
    // A service whose constructor forgets `makeObservable` still passes every behavioural test here,
    // because nothing in jest reacts to its state - and then does nothing at all in the running app.
    // Assert the annotations directly, which is the only place this is cheap to catch.
    const { service } = mockInjectedService(VisualsService);

    expect(isObservableProp(service, "visual")).toBe(true);
    expect(isObservableProp(service, "isReady")).toBe(true);
    expect(isComputedProp(service, "sourceLabel")).toBe(true);
  });
});

describe("VisualsService opening", () => {
  beforeEach(() => {
    resetMockInvoke();
  });

  it("builds views from the description and the buffer it describes", async () => {
    const { selected, buffer } = mockOpenableVisual();
    const { service } = mockInjectedService(VisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: selected,
      ["plugin:visuals|read_geometry"]: buffer,
    });

    await service.openFile("C:\\gamedata\\wpn_ak74.ogf");

    expect(service.visual.value?.views.submeshes).toHaveLength(1);
    expect(service.visual.error).toBeNull();
    expect(service.sourceLabel).toBe("C:\\gamedata\\wpn_ak74.ogf");
  });

  it("reports a failed open without leaving a stale model on screen", async () => {
    const { service } = mockInjectedService(VisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: () => {
        throw new Error("not an ogf file");
      },
    });

    await service.openFile("C:\\gamedata\\broken.ogf");

    expect(service.visual.value).toBeNull();
    expect(service.visual.error?.message).toBe("not an ogf file");
    expect(service.visual.isLoading).toBe(false);
  });

  it("restores whatever the backend still has selected", async () => {
    // A reload re-provisions the service, and the backend keeps the selection for exactly this reason.
    const { selected, buffer } = mockOpenableVisual("C:\\gamedata\\stalker.ogf");
    const { service } = mockInjectedService(VisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|get_model"]: selected,
      ["plugin:visuals|read_geometry"]: buffer,
    });

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.sourceLabel).toBe("C:\\gamedata\\stalker.ogf");
  });

  it("becomes ready with nothing open when the backend has no selection", async () => {
    const { service } = mockInjectedService(VisualsService);

    setMockInvokeResponses({ ["plugin:visuals|get_model"]: null });

    await service.onProvision();

    expect(service.isReady).toBe(true);
    expect(service.visual.value).toBeNull();
  });

  it("discards geometry for a visual the user has moved past", async () => {
    // Both calls are addressed by source, so a late response is identifiable. Pairing it with the current
    // description would upload one model's bytes under another's byte ranges.
    const first = mockOpenableVisual("C:\\gamedata\\first.ogf");
    const second = mockOpenableVisual("C:\\gamedata\\second.ogf");
    const { service } = mockInjectedService(VisualsService);

    let releaseFirstGeometry: Nullable<() => void> = null;

    const pendingFirst: Promise<ArrayBuffer> = new Promise((resolve) => {
      releaseFirstGeometry = () => resolve(first.buffer);
    });

    function isFirst(parameters?: Record<string, unknown>): boolean {
      return (parameters as { source: { path: string } }).source.path === first.selected.source.path;
    }

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: (parameters?: Record<string, unknown>) =>
        isFirst(parameters) ? first.selected : second.selected,
      ["plugin:visuals|read_geometry"]: (parameters?: Record<string, unknown>) =>
        isFirst(parameters) ? pendingFirst : second.buffer,
    });

    const opening: Promise<void> = service.openFile("C:\\gamedata\\first.ogf");

    await service.openFile("C:\\gamedata\\second.ogf");

    (releaseFirstGeometry as unknown as () => void)();
    await opening;

    expect(service.sourceLabel).toBe("C:\\gamedata\\second.ogf");
  });

  it("clears the model when closed", async () => {
    const { selected, buffer } = mockOpenableVisual();
    const { service } = mockInjectedService(VisualsService);

    setMockInvokeResponses({
      ["plugin:visuals|open_model"]: selected,
      ["plugin:visuals|read_geometry"]: buffer,
      ["plugin:visuals|close_model"]: null,
    });

    await service.openFile("C:\\gamedata\\wpn_ak74.ogf");
    await service.close();

    expect(service.visual.value).toBeNull();
    expect(service.sourceLabel).toBeNull();
  });
});
