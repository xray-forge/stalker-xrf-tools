import { beforeEach, describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { Route, Routes } from "react-router-dom";

import { ExportsEditorRouter } from "@/applications/exports-editor/ExportsEditorRouter";
import { ProjectService } from "@/core/store/project";
import { mockExportsProject } from "@/fixtures/mocks/project.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";
import { EExportsEditorCommand } from "@/lib/ipc";

describe("ExportsEditorRouter", () => {
  beforeEach(() => {
    window.localStorage.clear();
    window.localStorage.setItem("xrf-project-path", "C:\\projects\\active-xrf");

    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: null,
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: mockExportsProject({ root: "C:\\projects\\active-xrf" }),
    });
  });

  function renderRouter(route: string) {
    return renderWithProviders(
      <Routes>
        <Route path={"/exports-editor/*"} element={<ExportsEditorRouter />} />
        <Route path={"/"} element={<div>Application home</div>} />
      </Routes>,
      { route, bindings: [ProjectService] }
    );
  }

  it("shows tool selection before opening the separate exports form", async () => {
    const { findByDisplayValue, findByRole, findByText, queryByText } = renderRouter("/exports-editor");

    expect(await findByText("Open")).toBeInTheDocument();
    expect(queryByText("Open script exports")).not.toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, expect.anything());

    await userEvent.click(await findByText("Open"));

    expect(await findByText("Open script exports")).toBeInTheDocument();
    expect(await findByDisplayValue("C:\\projects\\active-xrf")).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, expect.anything());

    await userEvent.click(await findByRole("button", { name: "Open exports" }));

    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, {
      projectPath: "C:\\projects\\active-xrf",
    });
  });

  it("opens the exports form directly on its dedicated route", async () => {
    const { findByDisplayValue, findByText, queryByText } = renderRouter("/exports-editor/exports");

    expect(await findByText("Open script exports")).toBeInTheDocument();
    expect(await findByDisplayValue("C:\\projects\\active-xrf")).toBeInTheDocument();
    expect(queryByText("Browse extern declarations from an XRF project")).not.toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, expect.anything());
  });
});
