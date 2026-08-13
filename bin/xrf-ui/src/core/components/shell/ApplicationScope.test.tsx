import { describe, expect, it } from "@jest/globals";
import { Injectable } from "@wirestate/core";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ApplicationScope } from "@/core/components/shell/ApplicationScope";
import {
  EditorPanelsProvider,
  IEditorPanel,
  selectPanelsOnSide,
  useEditorPanels,
  useEditorPanelsRegistry,
} from "@/core/components/shell/panel/context";
import {
  EApplicationGroupId,
  EApplicationId,
  EApplicationStatus,
  IApplicationDescriptor,
} from "@/core/router/application";
import { renderWithProviders } from "@/fixtures/utils/render";

@Injectable()
class ScopedService {
  public readonly label: string = "scoped service";
}

const PANEL: IEditorPanel = {
  icon: <span>p</span>,
  id: "scoped",
  label: "Scoped",
  render: () => <ScopedPanel />,
  side: "left",
};

function ScopedPanel(): ReactElement {
  const service: ScopedService = useInjection(ScopedService);

  return <div>{service.label}</div>;
}

/** Publishes a panel and nothing else, the way an editor does. */
function Publisher(): ReactElement {
  useEditorPanels([PANEL]);

  return <div>content</div>;
}

/** Stands in for `ApplicationPanelSlot`, which is the thing that renders a published panel. */
function PanelSlot(): ReactElement {
  const panels: Array<IEditorPanel> = useEditorPanelsRegistry();

  return <>{selectPanelsOnSide(panels, "left").map((panel: IEditorPanel) => panel.render())}</>;
}

const APPLICATION: IApplicationDescriptor = {
  container: { bindings: [ScopedService] },
  Component: Publisher,
  description: "",
  group: EApplicationGroupId.ARCHIVES,
  icon: <span>a</span>,
  id: EApplicationId.ARCHIVES,
  label: "Scoped application",
  path: "/archives",
  status: EApplicationStatus.READY,
};

describe("ApplicationScope", () => {
  it("reaches the panels the shell renders, not just the application's own tree", () => {
    // The archives menu injects its service and is published as a panel. When the application provided
    // its own container the panel rendered outside it and the injection threw, which is the whole
    // reason bindings moved onto the descriptor.
    const { getByText } = renderWithProviders(
      <EditorPanelsProvider>
        <ApplicationScope application={APPLICATION}>
          <Publisher />
          <PanelSlot />
        </ApplicationScope>
      </EditorPanelsProvider>
    );

    expect(getByText("content")).toBeInTheDocument();
    expect(getByText("scoped service")).toBeInTheDocument();
  });

  it("renders in the root container when no application owns the route", () => {
    const { getByText } = renderWithProviders(
      <ApplicationScope application={null}>
        <div>home</div>
      </ApplicationScope>
    );

    expect(getByText("home")).toBeInTheDocument();
  });
});
