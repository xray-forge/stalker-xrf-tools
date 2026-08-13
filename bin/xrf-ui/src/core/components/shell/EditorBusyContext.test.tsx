import { describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { ReactElement, useState } from "react";

import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { EditorBusyProvider, useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { renderWithProviders } from "@/fixtures/utils/render";

function Busy({ isBusy }: { isBusy: boolean }): ReactElement {
  useEditorBusy(isBusy);

  return <div>editor</div>;
}

function Unmountable(): ReactElement {
  const [isMounted, setMounted] = useState(true);

  return (
    <>
      {isMounted ? <Busy isBusy /> : null}
      <button onClick={() => setMounted(false)}>unmount</button>
    </>
  );
}

/**
 * Against the toolbar rather than the rail: the rail used to carry a Home button and that was what a
 * running command blocked. Home is gone, so the toolbar's leaving control is the only way out of an
 * application and the one that has to stop.
 */
function renderToolbar(editor: ReactElement) {
  return renderWithProviders(
    <EditorBusyProvider>
      {editor}
      <EditorToolbar />
    </EditorBusyProvider>
  );
}

describe("useEditorBusy", () => {
  it("leaves navigation available when nothing is running", () => {
    const { getByText } = renderToolbar(<Busy isBusy={false} />);

    expect(getByText("XRF")).not.toBeDisabled();
  });

  it("blocks navigation while a command is running", () => {
    const { getByText } = renderToolbar(<Busy isBusy />);

    // Walking away used to leave the command running against a screen nobody could see.
    expect(getByText("XRF")).toBeDisabled();
  });

  it("releases the block when the editor unmounts, so a crash cannot strand the application", async () => {
    const { getByText } = renderToolbar(<Unmountable />);

    expect(getByText("XRF")).toBeDisabled();

    await userEvent.click(getByText("unmount"));

    expect(getByText("XRF")).not.toBeDisabled();
  });
});
