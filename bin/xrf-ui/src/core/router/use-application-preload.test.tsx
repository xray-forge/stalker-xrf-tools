import { describe, expect, it, jest } from "@jest/globals";

import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { APPLICATIONS } from "@/core/router/applications";
import { selectPreloadedApplications } from "@/core/router/use-application-preload";

function mockApplication(overrides: Partial<IApplicationDescriptor> = {}): IApplicationDescriptor {
  return {
    Component: () => null,
    description: "",
    group: EApplicationGroupId.SPAWNS,
    icon: <span />,
    id: EApplicationId.SPAWN,
    label: "Spawn",
    path: "/spawn",
    status: EApplicationStatus.READY,
    ...overrides,
  };
}

describe("selectPreloadedApplications", () => {
  it("warms an application that can be warmed", () => {
    const warmed: IApplicationDescriptor = mockApplication({ preload: jest.fn(async () => {}) });

    expect(selectPreloadedApplications([warmed])).toEqual([warmed]);
  });

  it("leaves a statically imported application alone", () => {
    // No `preload` is how a descriptor says its component is already in the initial graph, so there is
    // no chunk to fetch and nothing to warm.
    expect(selectPreloadedApplications([mockApplication()])).toEqual([]);
  });

  it("finds something to warm in the real roster", () => {
    expect(selectPreloadedApplications(APPLICATIONS).length).toBeGreaterThan(0);
  });

  it("only ever warms applications that carry the means to be warmed", () => {
    expect(
      selectPreloadedApplications(APPLICATIONS).every((it: IApplicationDescriptor) => typeof it.preload === "function")
    ).toBe(true);
  });
});
