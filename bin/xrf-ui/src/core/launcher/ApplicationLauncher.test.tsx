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
    id: EApplicationId.SPAWN_EDITOR,
    label: "Spawn editor",
    path: "/spawn-editor",
    status: EApplicationStatus.READY,
  },
  {
    Component: () => null,
    description: "Browse database archives",
    group: EApplicationGroupId.ARCHIVES,
    icon: <span />,
    id: EApplicationId.ARCHIVES_EXPLORER,
    label: "Archives editor",
    path: "/archives-explorer",
    status: EApplicationStatus.READY,
  },
];

const GROUPS: ReadonlyArray<IApplicationGroup> = [
  {
    accent: { light: "#000000", dark: "#ffffff" },
    id: EApplicationGroupId.ARCHIVES,
    icon: <span />,
    label: "Archives",
  },
  {
    accent: { light: "#000000", dark: "#ffffff" },
    id: EApplicationGroupId.SPAWNS,
    icon: <span />,
    label: "Spawns",
  },
];

function renderLauncher() {
  return renderWithProviders(<ApplicationLauncher applications={APPLICATIONS} groups={GROUPS} />);
}

describe("ApplicationLauncher", () => {
  it("packs the caller's catalog in stable group order", () => {
    const { getAllByRole, getByText } = renderLauncher();

    expect(getByText("2 tools across 2 groups")).toBeInTheDocument();
    expect(getByText("Archives editor")).toBeInTheDocument();
    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(
      getAllByRole("button")
        .map((button: HTMLElement) => button.getAttribute("aria-label"))
        .filter((label: string | null): label is string => label !== null)
    ).toEqual(["Archives editor", "Spawn editor"]);
  });

  it("heads each group's run of cards, so the taxonomy is visible without reading colours", () => {
    const { getAllByRole } = renderLauncher();

    expect(getAllByRole("heading", { level: 2 }).map((heading: HTMLElement) => heading.textContent)).toEqual([
      "Archives",
      "Spawns",
    ]);
  });

  it("drops the sections while searching and ranks what is left", async () => {
    const { getByLabelText, getByText, queryByRole, queryByText } = renderLauncher();

    await userEvent.type(getByLabelText("Search tools"), "spawn");

    expect(getByText("1 match")).toBeInTheDocument();
    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(queryByText("Archives editor")).not.toBeInTheDocument();
    expect(queryByRole("heading", { level: 2 })).not.toBeInTheDocument();
  });

  it("matches a group name that no label of its own mentions", async () => {
    const { getByLabelText, getByText, queryByText } = renderLauncher();

    await userEvent.type(getByLabelText("Search tools"), "spawns");

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(queryByText("Archives editor")).not.toBeInTheDocument();
  });

  it("says so rather than showing an empty grid when nothing matches", async () => {
    const { getByLabelText, getByText } = renderLauncher();

    await userEvent.type(getByLabelText("Search tools"), "nothing-here");

    expect(getByText("No tools match")).toBeInTheDocument();
  });

  it("narrows to one group by chip, and the same chip lets go again", async () => {
    const { getByRole, getByText, queryByText } = renderLauncher();

    await userEvent.click(getByRole("button", { name: "Archives 1" }));

    expect(getByText("Archives editor")).toBeInTheDocument();
    expect(queryByText("Spawn editor")).not.toBeInTheDocument();

    await userEvent.click(getByRole("button", { name: "Archives 1" }));

    expect(getByText("Spawn editor")).toBeInTheDocument();
  });

  it("scopes a search to the chosen group rather than replacing it", async () => {
    const { getByLabelText, getByRole, getByText, queryByText } = renderLauncher();

    await userEvent.click(getByRole("button", { name: "Archives 1" }));
    await userEvent.type(getByLabelText("Search tools"), "editor");

    expect(getByText("Archives editor")).toBeInTheDocument();
    expect(queryByText("Spawn editor")).not.toBeInTheDocument();
  });

  it("focuses the search field from the keyboard, without a pointer", async () => {
    const { getByLabelText } = renderLauncher();

    await userEvent.keyboard("{Control>}k{/Control}");

    expect(getByLabelText("Search tools")).toHaveFocus();
  });
});
