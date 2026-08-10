import { describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { ReactElement, useState } from "react";

import { ApplicationRail } from "@/core/components/shell/ApplicationRail";
import { EditorBusyProvider, useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { renderWithProviders } from "@/fixtures/render";

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

function renderRail(editor: ReactElement) {
  return renderWithProviders(
    <EditorBusyProvider>
      {editor}
      <ApplicationRail />
    </EditorBusyProvider>
  );
}

describe("useEditorBusy", () => {
  it("leaves navigation available when nothing is running", () => {
    const { getByLabelText } = renderRail(<Busy isBusy={false} />);

    expect(getByLabelText("Spawns")).not.toBeDisabled();
    expect(getByLabelText("Home")).not.toBeDisabled();
  });

  it("blocks navigation while a command is running", () => {
    const { getByLabelText } = renderRail(<Busy isBusy />);

    // Walking away used to leave the command running against a screen nobody could see.
    expect(getByLabelText("Spawns")).toBeDisabled();
    expect(getByLabelText("Home")).toBeDisabled();
  });

  it("leaves controls that abandon nothing alone", () => {
    const { getByLabelText } = renderRail(<Busy isBusy />);

    expect(getByLabelText("Settings")).not.toBeDisabled();
    expect(getByLabelText("Source on github")).not.toBeDisabled();
  });

  it("releases the block when the editor unmounts, so a crash cannot strand the application", async () => {
    const { getByLabelText, getByText } = renderRail(<Unmountable />);

    expect(getByLabelText("Spawns")).toBeDisabled();

    await userEvent.click(getByText("unmount"));

    expect(getByLabelText("Spawns")).not.toBeDisabled();
  });
});
