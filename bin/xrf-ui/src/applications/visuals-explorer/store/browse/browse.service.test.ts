import { beforeEach, describe, expect, it } from "@jest/globals";
import { isComputedProp, isObservableProp } from "@wirestate/mobx";

import { VisualsBrowseService } from "@/applications/visuals-explorer/store/browse";
import { XrayAsset } from "@/core/bindings/types/xrf-vfs";
import { resetMockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { Nullable } from "@/lib/types/general";

function mockVisual(logicalPath: string): XrayAsset {
  return {
    container: { kind: "directory", relativePath: logicalPath, root: "C:\\gamedata" },
    logicalPath,
  };
}

describe("VisualsBrowseService", () => {
  beforeEach(() => {
    resetMockInvoke();
    window.localStorage.clear();
  });

  it("applies its mobx annotations", () => {
    const { service } = mockInjectedService(VisualsBrowseService);

    expect(isObservableProp(service, "root")).toBe(true);
    expect(isObservableProp(service, "visuals")).toBe(true);
    expect(isComputedProp(service, "isBrowsing")).toBe(true);
    expect(isComputedProp(service, "roots")).toBe(true);
  });

  it("lists every visual of the root it browses, asking for models only", async () => {
    const { service } = mockInjectedService(VisualsBrowseService);

    let listParameters: Nullable<Record<string, unknown>> = null;

    setMockInvokeResponses({
      ["plugin:assets|list_assets"]: (parameters?: Record<string, unknown>) => {
        listParameters = parameters ?? null;

        return [mockVisual("meshes\\wpn\\wpn_ak74.ogf")];
      },
    });

    await service.openRoot("C:\\gamedata");

    // No subject asset: a browsed root is the world itself, not a neighbourhood around one model.
    expect(listParameters).toEqual({ kind: "ogf", world: { asset: null, roots: ["C:\\gamedata"] } });
    expect(service.visuals.value).toHaveLength(1);
    expect(service.isBrowsing).toBe(true);
    expect(service.roots).toEqual(["C:\\gamedata"]);
  });

  it("reports a failed listing without pretending the root is empty of intent", async () => {
    const { service } = mockInjectedService(VisualsBrowseService);

    setMockInvokeResponses({
      ["plugin:assets|list_assets"]: () => {
        throw new Error("root does not exist");
      },
    });

    await service.openRoot("C:\\missing");

    expect(service.visuals.error?.message).toBe("root does not exist");
    expect(service.visuals.isLoading).toBe(false);
    expect(service.isBrowsing).toBe(true);
  });

  it("comes back to the root it was browsing after a reload", async () => {
    // The tree is frontend state, so provisioning re-lists rather than restoring a backend session; the mounts the
    // listing needs are already cached, so repeating it indexes nothing new.
    setMockInvokeResponses({ ["plugin:assets|list_assets"]: [mockVisual("meshes\\actors\\stalker.ogf")] });

    const first = mockInjectedService(VisualsBrowseService);

    await first.service.openRoot("C:\\gamedata");

    const second = mockInjectedService(VisualsBrowseService);

    await second.service.onProvision();

    expect(second.service.root).toBe("C:\\gamedata");
    expect(second.service.visuals.value).toHaveLength(1);
  });

  it("forgets the root on deactivation, so leaving closes in place", async () => {
    // The selection is dropped on the way out, so a remembered tree would come back beside a model the backend no
    // longer has. A reload runs no deactivation, which is what keeps the restore above working.
    setMockInvokeResponses({ ["plugin:assets|list_assets"]: [mockVisual("meshes\\actors\\stalker.ogf")] });

    const { service } = mockInjectedService(VisualsBrowseService);

    await service.openRoot("C:\\gamedata");

    service.onDeactivation();

    expect(service.root).toBeNull();
    expect(service.visuals.value).toEqual([]);

    const next = mockInjectedService(VisualsBrowseService);

    await next.service.onProvision();

    expect(next.service.root).toBeNull();
  });

  it("forgets the root when browsing is closed", async () => {
    setMockInvokeResponses({ ["plugin:assets|list_assets"]: [] });

    const { service } = mockInjectedService(VisualsBrowseService);

    await service.openRoot("C:\\gamedata");
    service.close();

    expect(service.root).toBeNull();
    expect(service.isBrowsing).toBe(false);
    expect(service.roots).toEqual([]);

    const next = mockInjectedService(VisualsBrowseService);

    await next.service.onProvision();

    expect(next.service.root).toBeNull();
  });
});
