import { describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { ApplicationLauncher } from "@/core/launcher/ApplicationLauncher";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
  IApplicationGroup,
} from "@/core/routing/application";
import { renderWithProviders } from "@/fixtures/utils/render";

const APPLICATIONS: ReadonlyArray<IApplicationDescriptor> = [
  {
    Component: () => null,
    description: "Browse packed spawn files",
    group: EApplicationGroupId.SPAWNS,
    icon: <span />,
    id: EApplicationId.SPAWN,
    label: "Spawn editor",
    path: "/spawn",
    status: EApplicationStatus.READY,
  },
  {
    Component: () => null,
    description: "Browse database archives",
    group: EApplicationGroupId.ARCHIVES,
    icon: <span />,
    id: EApplicationId.ARCHIVES,
    label: "Archives editor",
    path: "/archives",
    status: EApplicationStatus.READY,
  },
];

const GROUPS: ReadonlyArray<IApplicationGroup> = [
  { id: EApplicationGroupId.ARCHIVES, icon: <span />, label: "Archives" },
  { id: EApplicationGroupId.SPAWNS, icon: <span />, label: "Spawns" },
];

describe("ApplicationLauncher", () => {
  it("renders and filters the catalog supplied by its caller", async () => {
    const { getByLabelText, getByText, queryByText } = renderWithProviders(
      <ApplicationLauncher applications={APPLICATIONS} groups={GROUPS} />
    );

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(getByText("Archives editor")).toBeInTheDocument();

    await userEvent.type(getByLabelText("Filter applications"), "database");

    expect(queryByText("Spawn editor")).not.toBeInTheDocument();
    expect(getByText("Archives editor")).toBeInTheDocument();
    expect(queryByText("Spawns")).not.toBeInTheDocument();
  });
});
