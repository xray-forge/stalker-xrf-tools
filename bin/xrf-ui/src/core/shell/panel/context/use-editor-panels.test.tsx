import { describe, expect, it } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";
import { Fragment, ReactElement, useState } from "react";

import { IEditorPanel, useEditorPanels, useEditorPanelsRegistry } from "@/core/shell/panel/context/index";
import { renderWithProviders } from "@/fixtures/utils/render";

function DynamicPublisher(): ReactElement {
  const [label, setLabel] = useState<string>("First");

  useEditorPanels(
    () => [
      {
        icon: <span>p</span>,
        id: "dynamic",
        label: "Dynamic",
        render: () => <div>{label}</div>,
      },
    ],
    [label]
  );

  return <button onClick={() => setLabel("Second")}>Update panel</button>;
}

function PanelOutlet(): ReactElement {
  const panels: ReadonlyArray<IEditorPanel> = useEditorPanelsRegistry();

  return (
    <>
      {panels.map((panel: IEditorPanel) => (
        <Fragment key={panel.id}>{panel.render()}</Fragment>
      ))}
    </>
  );
}

describe("useEditorPanels", () => {
  it("publishes a new renderer when the panel array changes", async () => {
    const { findByText, getByText, queryByText } = renderWithProviders(
      <>
        <DynamicPublisher />
        <PanelOutlet />
      </>
    );

    expect(await findByText("First")).toBeInTheDocument();

    await userEvent.click(getByText("Update panel"));

    expect(await findByText("Second")).toBeInTheDocument();
    expect(queryByText("First")).not.toBeInTheDocument();
  });
});
