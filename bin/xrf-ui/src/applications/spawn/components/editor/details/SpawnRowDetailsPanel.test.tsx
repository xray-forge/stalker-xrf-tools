import { describe, expect, it } from "@jest/globals";
import { act, RenderResult } from "@testing-library/react";

import { SpawnRowDetailsPanel } from "@/applications/spawn/components/editor/details/SpawnRowDetailsPanel";
import { SpawnFileService } from "@/core/spawn/services";
import { mockAlifeObject } from "@/fixtures/mocks/spawn.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { renderWithProviders } from "@/fixtures/utils/render";
import { AnyObject } from "@/lib/types/general";

interface IPanelRender {
  render: RenderResult;
  service: SpawnFileService;
}

/** Seeded before rendering, which is the order the panel meets a selection when it is opened. */
function renderPanel(selected?: AnyObject): IPanelRender {
  const { service }: { service: SpawnFileService } = mockInjectedService(SpawnFileService);

  if (selected) {
    service.selectRow("Alife object", 0, selected);
  }

  return { render: renderWithProviders(<SpawnRowDetailsPanel spawnFileService={service} />), service };
}

describe("SpawnRowDetailsPanel", () => {
  it("asks for a selection rather than showing an empty frame", () => {
    const { render }: IPanelRender = renderPanel();

    expect(render.getByText("Nothing selected")).toBeInTheDocument();
  });

  it("names what kind of row is showing", () => {
    const { render }: IPanelRender = renderPanel(mockAlifeObject());

    expect(render.getByText("Alife object")).toBeInTheDocument();
  });

  it("shows the fields the table columns deliberately leave out", () => {
    const { render }: IPanelRender = renderPanel(mockAlifeObject());

    // `inherited` and `updateData` are not columns, which is the point of the panel.
    expect(render.getByText("inherited")).toBeInTheDocument();
    expect(render.getByText("updateData")).toBeInTheDocument();
  });

  it("renders a vector readably rather than as JSON", () => {
    const { render }: IPanelRender = renderPanel(mockAlifeObject());

    expect(render.getByText("x: 12.5, y: 1.25, z: -30")).toBeInTheDocument();
  });

  it("says an empty list is empty rather than printing nothing", () => {
    const { render }: IPanelRender = renderPanel(mockAlifeObject());

    expect(render.getByText("empty")).toBeInTheDocument();
  });

  it("drops the selection when the file it pointed into closes", async () => {
    const { render, service }: IPanelRender = renderPanel(mockAlifeObject());

    await act(() => service.closeSpawnFile());

    // A selection outliving its data would render a row from a file that is no longer open.
    expect(render.getByText("Nothing selected")).toBeInTheDocument();
  });
});
