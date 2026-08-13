import { describe, expect, it, jest } from "@jest/globals";
import { fireEvent } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { ApplicationCard } from "@/core/components/navigation/ApplicationCard";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/routing/application";
import { renderWithProviders } from "@/fixtures/utils/render";

function mockApplication(overrides: Partial<IApplicationDescriptor> = {}): IApplicationDescriptor {
  return {
    Component: () => null,
    description: "Browse and edit a packed spawn file",
    group: EApplicationGroupId.SPAWNS,
    icon: <span />,
    id: EApplicationId.SPAWN,
    label: "Spawn editor",
    path: "/spawn",
    status: EApplicationStatus.READY,
    ...overrides,
  };
}

describe("ApplicationCard", () => {
  it("warms the chunk when the pointer arrives, before any click", async () => {
    const preload = jest.fn(async () => {});

    const { getByRole } = renderWithProviders(
      <ApplicationCard application={mockApplication({ preload })} isEnabled onOpen={jest.fn()} />
    );

    await userEvent.hover(getByRole("button"));

    // Intent runs ahead of the click, which is the whole point: the fetch is already in flight.
    expect(preload).toHaveBeenCalledTimes(1);
  });

  it("warms on keyboard focus too, so the mouse is not the only way in", async () => {
    const preload = jest.fn(async () => {});

    const { getByRole } = renderWithProviders(
      <ApplicationCard application={mockApplication({ preload })} isEnabled onOpen={jest.fn()} />
    );

    await userEvent.tab();

    expect(getByRole("button")).toHaveFocus();
    expect(preload).toHaveBeenCalled();
  });

  it("does not warm an application you cannot open", () => {
    const preload = jest.fn(async () => {});

    const { getByRole } = renderWithProviders(
      <ApplicationCard
        application={mockApplication({ preload, status: EApplicationStatus.PLANNED })}
        isEnabled={false}
        onOpen={jest.fn()}
      />
    );

    // Dispatched rather than hovered: a disabled card carries `pointer-events: none`, so a real pointer
    // never reaches it and the guard would go untested behind that.
    fireEvent.mouseEnter(getByRole("button"));

    expect(preload).not.toHaveBeenCalled();
  });

  it("survives a statically imported application, which has nothing to warm", async () => {
    const { getByRole } = renderWithProviders(
      <ApplicationCard application={mockApplication()} isEnabled onOpen={jest.fn()} />
    );

    await userEvent.hover(getByRole("button"));

    expect(getByRole("button")).toBeInTheDocument();
  });

  it("still opens on click", async () => {
    const onOpen = jest.fn();

    const { getByRole } = renderWithProviders(
      <ApplicationCard application={mockApplication()} isEnabled onOpen={onOpen} />
    );

    await userEvent.click(getByRole("button"));

    expect(onOpen).toHaveBeenCalledTimes(1);
  });
});
