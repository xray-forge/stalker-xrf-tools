import { beforeEach, describe, expect, it } from "@jest/globals";
import { RenderResult } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { ReactElement, ReactNode } from "react";

import { ApplicationShellFrame } from "@/core/components/shell/ApplicationShellFrame";
import { EditorPanelsProvider, useEditorPanels } from "@/core/components/shell/panel/context";
import { NotificationsService } from "@/core/store/notifications";
import { renderWithProviders } from "@/fixtures/utils/render";

/** Stands in for an editor that publishes one default-open panel, which most of them do. */
function EditorWithPanel({ name }: { name: string }): ReactElement {
  useEditorPanels([{ icon: <span>{name}</span>, id: name, label: name, render: () => <div>{name} panel</div> }]);

  return <div>{name} editor</div>;
}

function renderFrame(children: ReactNode): RenderResult {
  return renderWithProviders(
    <EditorPanelsProvider>
      <ApplicationShellFrame>{children}</ApplicationShellFrame>
    </EditorPanelsProvider>,
    { bindings: [NotificationsService] }
  );
}

describe("ApplicationShellFrame", () => {
  beforeEach(() => {
    // The frame remembers which panel was open, so a leftover choice would decide the next test.
    window.localStorage.clear();
  });

  it("offers the notification centre even when the active editor declares no panels", () => {
    const { getByLabelText } = renderFrame(<div>bare editor</div>);

    expect(getByLabelText("Notifications")).toBeInTheDocument();
  });

  it("gives the slot to the global panel over an editor's default-open one", async () => {
    const { getByLabelText, getByText, queryByText } = renderFrame(<EditorWithPanel name={"Bones"} />);

    expect(getByText("Bones panel")).toBeInTheDocument();

    await userEvent.click(getByLabelText("Notifications"));

    // One slot, two claimants: an explicit click outranks a default.
    expect(getByText(/Nothing has been reported yet/)).toBeInTheDocument();
    expect(queryByText("Bones panel")).not.toBeInTheDocument();
  });

  it("keeps the global panel open when the editor under it changes", async () => {
    const { getByLabelText, getByText, queryByText, rerender } = renderFrame(<EditorWithPanel name={"Bones"} />);

    await userEvent.click(getByLabelText("Notifications"));

    rerender(
      <EditorPanelsProvider>
        <ApplicationShellFrame>
          <EditorWithPanel name={"Header"} />
        </ApplicationShellFrame>
      </EditorPanelsProvider>
    );

    expect(getByText("Header editor")).toBeInTheDocument();
    expect(getByText(/Nothing has been reported yet/)).toBeInTheDocument();
    expect(queryByText("Header panel")).not.toBeInTheDocument();
  });

  it("releases the slot when an editor panel is picked instead", async () => {
    const { getByLabelText, getByText, queryByText } = renderFrame(<EditorWithPanel name={"Bones"} />);

    await userEvent.click(getByLabelText("Notifications"));
    await userEvent.click(getByLabelText("Bones"));

    expect(getByText("Bones panel")).toBeInTheDocument();
    expect(queryByText(/Nothing has been reported yet/)).not.toBeInTheDocument();
  });

  it("collapses the global panel when its own control is clicked again", async () => {
    const { getByLabelText, queryByText } = renderFrame(<EditorWithPanel name={"Bones"} />);

    await userEvent.click(getByLabelText("Notifications"));
    await userEvent.click(getByLabelText("Notifications"));

    // Back to the editor's own default, which is what the slot held before the global panel took it.
    expect(queryByText(/Nothing has been reported yet/)).not.toBeInTheDocument();
    expect(queryByText("Bones panel")).toBeInTheDocument();
  });
});
