import { beforeEach, describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { Route, Routes } from "react-router-dom";

import { ExportsApplication } from "@/applications/exports/ExportsApplication";
import { ProjectService } from "@/core/store/project";
import { mockExportsProject } from "@/fixtures/mocks/project.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";
import { EExportsEditorCommand } from "@/lib/ipc";

describe("ExportsApplication", () => {
  beforeEach(() => {
    window.localStorage.clear();
    window.localStorage.setItem("xrf-project-path", "C:\\projects\\active-xrf");

    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: null,
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: mockExportsProject({ root: "C:\\projects\\active-xrf" }),
    });
  });

  function renderApplication(route: string) {
    return renderWithProviders(
      <Routes>
        <Route path={"/exports/*"} element={<ExportsApplication />} />
        <Route path={"/"} element={<div>Application home</div>} />
      </Routes>,
      { route, bindings: [ProjectService] }
    );
  }

  it("lands on its own picker, with no list of one thing in between", async () => {
    // The route used to open a landing pane holding a single card called "Open". Flattening deleted
    // that pane: the application is the thing home links to, so it opens what it is for.
    const { findByDisplayValue, findByText, queryByText } = renderApplication("/exports");

    expect(await findByText("Open script exports")).toBeInTheDocument();
    expect(await findByDisplayValue("C:\\projects\\active-xrf")).toBeInTheDocument();
    expect(queryByText("Open")).not.toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, expect.anything());
  });

  it("opens the project the picker was given", async () => {
    const { findByRole } = renderApplication("/exports");

    await userEvent.click(await findByRole("button", { name: "Open exports" }));

    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, {
      projectPath: "C:\\projects\\active-xrf",
    });
  });

  it("provides its own container, so it needs nothing bound above it", async () => {
    const { findByText } = renderApplication("/exports");

    expect(await findByText("Open script exports")).toBeInTheDocument();
  });
});
