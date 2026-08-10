import { beforeEach, describe, expect, it } from "@jest/globals";
import { RenderResult } from "@testing-library/react";
import { Route, Routes } from "react-router-dom";

import { SpawnEditor } from "@/applications/spawn-editor/components/editor/SpawnEditor";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { renderWithProviders } from "@/fixtures/render";
import { mockSpawnFile } from "@/fixtures/spawn.mocks";
import { setMockInvokeResponses } from "@/fixtures/tauri.mocks";
import { ESpawnsEditorCommand } from "@/lib/ipc";

describe("SpawnEditor", () => {
  beforeEach(() => {
    setMockInvokeResponses({ [ESpawnsEditorCommand.GET_SPAWN_FILE]: mockSpawnFile() });
  });

  function renderEditor(): RenderResult {
    return renderWithProviders(
      <>
        <Routes>
          <Route path={"/spawn-editor/editor/*"} element={<SpawnEditor />} />
        </Routes>
        <ApplicationStatusBar />
      </>,
      { route: "/spawn-editor/editor/header", bindings: [SpawnFileService] }
    );
  }

  it("titles itself from the route, not from a hardcoded string", async () => {
    const { findByText } = renderEditor();

    expect(await findByText("Spawn editor")).toBeInTheDocument();
  });

  it("offers every chunk section in the side menu", async () => {
    const { findByText } = renderEditor();

    for (const section of ["Header", "Alife", "Artefacts", "Patrols", "Graph"]) {
      expect(await findByText(section)).toBeInTheDocument();
    }
  });

  it("keeps file actions in the menu and leaving in the toolbar", async () => {
    const { findByText, getByLabelText, getByText, queryByText } = renderEditor();

    expect(await findByText("Save")).toBeInTheDocument();
    expect(getByText("Export")).toBeInTheDocument();
    // Close moved onto the toolbar's back control, so it must no longer appear as a menu entry.
    expect(queryByText("Close")).not.toBeInTheDocument();
    expect(getByLabelText("Close and go back")).toBeInTheDocument();
  });

  it("publishes header counts to the status bar", async () => {
    const { findByText, getByText } = renderEditor();

    expect(await findByText("version 124")).toBeInTheDocument();
    expect(getByText("2 objects")).toBeInTheDocument();
    expect(getByText("1 levels")).toBeInTheDocument();
  });

  it("reports nothing when the backend has no file open", async () => {
    setMockInvokeResponses({ [ESpawnsEditorCommand.GET_SPAWN_FILE]: null });

    const { findByText, queryByText } = renderEditor();

    expect(await findByText("Ready")).toBeInTheDocument();
    expect(queryByText("2 objects")).not.toBeInTheDocument();
  });

  it("reflects the fixture rather than a fixed string", async () => {
    setMockInvokeResponses({
      [ESpawnsEditorCommand.GET_SPAWN_FILE]: mockSpawnFile({
        header: {
          version: 118,
          guid: "guid",
          graphGuid: "graph-guid",
          objectsCount: 9001,
          levelsCount: 32,
        },
      }),
    });

    const { findByText, getByText } = renderEditor();

    expect(await findByText("9001 objects")).toBeInTheDocument();
    expect(getByText("version 118")).toBeInTheDocument();
    expect(getByText("32 levels")).toBeInTheDocument();
  });

  it("marks the active chunk section", async () => {
    const { findByText } = renderEditor();

    const header: HTMLElement = await findByText("Header");

    expect(header.closest("[role='button']")).toHaveClass("Mui-selected");
  });
});
