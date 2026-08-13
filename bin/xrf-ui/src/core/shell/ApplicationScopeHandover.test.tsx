import { beforeEach, describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";
import { NavigateFunction, Route, Routes, useNavigate } from "react-router-dom";

import { ArchivesService } from "@/applications/archives/services/archives";
import { ApplicationShell } from "@/core/shell/ApplicationShell";
import { useEditorPanels } from "@/core/shell/panel/context";
import { setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";

/** Injects the way the archives menu does, which is what makes the handover observable. */
function ArchivesScopedPanel(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  return <div>{`archives panel ${archivesService.isReady}`}</div>;
}

/** Publishes a left panel bound to its own application's container, as the archives editor does. */
function ArchivesLikeEditor(): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  useEditorPanels(
    () => [
      {
        icon: <span>a</span>,
        id: "archives-menu",
        isOpenByDefault: true,
        label: "Archives",
        render: () => <ArchivesScopedPanel />,
        side: "left",
      },
    ],
    []
  );

  return <button onClick={() => navigate("/spawn", { replace: true })}>leave</button>;
}

describe("panel handover between applications", () => {
  beforeEach(() => {
    window.localStorage.clear();
    setMockInvokeResponses({});
  });

  it("stops rendering an application's panels the moment its container goes away", async () => {
    // The registry is cleared by an effect, but the container is swapped during render. For the commit
    // in between, the frame held the outgoing application's panels and the incoming one's container -
    // so a panel that injects asked a container that never bound its service.
    const { getByText, findByText, queryByText } = renderWithProviders(
      <ApplicationShell>
        <Routes>
          <Route path={"/archives/*"} element={<ArchivesLikeEditor />} />
          <Route path={"/spawn/*"} element={<div>spawn editor</div>} />
        </Routes>
      </ApplicationShell>,
      { route: "/archives" }
    );

    expect(await findByText(/archives panel/)).toBeInTheDocument();

    await userEvent.click(getByText("leave"));

    expect(await findByText("spawn editor")).toBeInTheDocument();
    expect(queryByText(/archives panel/)).not.toBeInTheDocument();
  });
});
