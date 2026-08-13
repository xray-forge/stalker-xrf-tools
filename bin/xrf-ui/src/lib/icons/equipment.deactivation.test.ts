import { beforeEach, describe, expect, it } from "@jest/globals";

import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { EquipmentService } from "@/lib/icons/equipment.service";
import { EIconsEditorCommand } from "@/lib/xrf/ipc";

function closeCalls(): number {
  return mockInvoke.mock.calls.filter(([command]) => command === EIconsEditorCommand.CLOSE_EQUIPMENT_SPRITE).length;
}

/**
 * These assert the container semantics the release hook depends on, not just that the hook exists.
 */
describe("EquipmentService deactivation", () => {
  beforeEach(() => {
    setMockInvokeResponses({});
  });

  it("does not release on deprovision alone, which strict mode reaches on every mount", async () => {
    const { container } = mockInjectedService(EquipmentService);

    await container.provision();

    container.get(EquipmentService);
    container.deprovision();

    // The strict mode remount cancels the pending `unbindAll`, so this is the whole teardown it sees.
    // Releasing here would close a project the user is still looking at.
    expect(closeCalls()).toBe(0);
  });

  it("releases once the container is actually unbound", async () => {
    const { container } = mockInjectedService(EquipmentService);

    await container.provision();

    container.get(EquipmentService);

    container.deprovision();
    container.unbindAll();

    expect(closeCalls()).toBe(1);
  });

  it("survives a strict mode style remount without releasing", async () => {
    const { container } = mockInjectedService(EquipmentService);

    await container.provision();

    container.get(EquipmentService);

    // Mount, throwaway unmount, remount - `unbindAll` never runs because the provider cancels it.
    container.deprovision();

    await container.provision();

    container.get(EquipmentService);

    expect(closeCalls()).toBe(0);

    // Leaving for real still releases.
    container.deprovision();
    container.unbindAll();

    expect(closeCalls()).toBe(1);
  });
});
