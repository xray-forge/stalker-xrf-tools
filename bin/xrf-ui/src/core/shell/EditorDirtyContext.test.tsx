import { describe, expect, it, jest } from "@jest/globals";
import { waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { ReactElement } from "react";

import { EditorDirtyProvider, useEditorDirty, useRequestLeave } from "@/core/shell/EditorDirtyContext";
import { renderWithProviders } from "@/fixtures/utils/render";

function Leaver({ dirtyCount, onLeave }: { dirtyCount: number; onLeave: () => void }): ReactElement {
  const requestLeave: (leave: () => void) => void = useRequestLeave();

  useEditorDirty(dirtyCount);

  return (
    <button type={"button"} onClick={() => requestLeave(onLeave)}>
      Leave
    </button>
  );
}

describe("EditorDirtyContext", () => {
  it("leaves immediately when nothing is unsaved", async () => {
    const onLeave = jest.fn();

    const { getByText } = renderWithProviders(
      <EditorDirtyProvider>
        <Leaver dirtyCount={0} onLeave={onLeave} />
      </EditorDirtyProvider>
    );

    await userEvent.click(getByText("Leave"));

    // No dialog is built at all for an editor holding nothing, which is every editor but this one.
    expect(onLeave).toHaveBeenCalledTimes(1);
  });

  it("asks before discarding unsaved work, and says how much", async () => {
    const onLeave = jest.fn();

    const { getByText } = renderWithProviders(
      <EditorDirtyProvider>
        <Leaver dirtyCount={3} onLeave={onLeave} />
      </EditorDirtyProvider>
    );

    await userEvent.click(getByText("Leave"));

    expect(getByText("Leave without saving?")).toBeInTheDocument();
    expect(getByText(/3 files have edits/)).toBeInTheDocument();
    expect(onLeave).not.toHaveBeenCalled();
  });

  it("stays put when the prompt is declined", async () => {
    const onLeave = jest.fn();

    const { getByText, queryByText } = renderWithProviders(
      <EditorDirtyProvider>
        <Leaver dirtyCount={1} onLeave={onLeave} />
      </EditorDirtyProvider>
    );

    await userEvent.click(getByText("Leave"));
    await userEvent.click(getByText("Stay"));

    expect(onLeave).not.toHaveBeenCalled();
    // The dialog animates out, so it is still in the tree for a frame after the click.
    await waitFor(() => expect(queryByText("Leave without saving?")).not.toBeInTheDocument());
  });

  it("goes through once the discard is confirmed", async () => {
    const onLeave = jest.fn();

    const { getByText } = renderWithProviders(
      <EditorDirtyProvider>
        <Leaver dirtyCount={1} onLeave={onLeave} />
      </EditorDirtyProvider>
    );

    await userEvent.click(getByText("Leave"));
    await userEvent.click(getByText("Discard and leave"));

    expect(onLeave).toHaveBeenCalledTimes(1);
  });

  it("phrases a single file as one", async () => {
    const { getByText } = renderWithProviders(
      <EditorDirtyProvider>
        <Leaver dirtyCount={1} onLeave={jest.fn()} />
      </EditorDirtyProvider>
    );

    await userEvent.click(getByText("Leave"));

    expect(getByText(/1 file has edits/)).toBeInTheDocument();
  });
});
