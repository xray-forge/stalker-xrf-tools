import { beforeEach, describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { Route, Routes } from "react-router-dom";

import { ExportsApplication } from "@/applications/exports/ExportsApplication";
import { ApplicationShell } from "@/core/components/shell/ApplicationShell";
import { ProjectService } from "@/core/services/project";
import { mockExportsProject } from "@/fixtures/mocks/project.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";
import { EExportsEditorCommand } from "@/lib/xrf/ipc";

describe("ExportsApplication", () => {
  beforeEach(() => {
    window.localStorage.clear();
    window.localStorage.setItem("xrf-project-path", "C:\\projects\\active-xrf");

    setMockInvokeResponses({
      [EExportsEditorCommand.GET_XR_EXPORTS]: null,
      [EExportsEditorCommand.OPEN_XR_EXPORTS]: mockExportsProject({ root: "C:\\projects\\active-xrf" }),
    });
  });

  /**
   * Through the shell rather than on its own: the container an application's services live in is built
   * by the frame from the descriptor, so mounting the component alone would prove nothing about how it
   * is actually wired.
   */
  function renderApplication(route: string) {
    return renderWithProviders(
      <ApplicationShell>
        <Routes>
          <Route path={"/exports/*"} element={<ExportsApplication />} />
          <Route path={"/"} element={<div>Application home</div>} />
        </Routes>
      </ApplicationShell>,
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

  it("resolves the services its descriptor declares, with nothing bound above the shell", async () => {
    // `ExportsService` is bound by the frame out of `EXPORTS_APPLICATION.bindings`. Only
    // `ProjectService` is provided here, so if that wiring broke this would throw rather than render.
    const { findByRole } = renderApplication("/exports");

    await userEvent.click(await findByRole("button", { name: "Open exports" }));

    expect(mockInvoke).toHaveBeenCalledWith(EExportsEditorCommand.OPEN_XR_EXPORTS, {
      projectPath: "C:\\projects\\active-xrf",
    });
  });
});
