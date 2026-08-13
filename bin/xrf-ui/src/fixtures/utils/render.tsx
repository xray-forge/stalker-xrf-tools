import { render, RenderResult } from "@testing-library/react";
import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { Fragment, PropsWithChildren, ReactElement, ReactNode } from "react";
import { MemoryRouter } from "react-router-dom";

import { ApplicationProvider } from "@/ApplicationProvider";
import { NotificationsService } from "@/core/notifications/services";
import { SettingsService } from "@/core/settings/services/settings";
import { EditorBusyProvider } from "@/core/shell/EditorBusyContext";
import { EditorStatusProvider } from "@/core/shell/EditorStatusContext";
import {
  EditorPanelsProvider,
  IEditorPanel,
  selectPanelsOnSide,
  useEditorPanelsRegistry,
} from "@/core/shell/panel/context";

export interface IRenderOptions {
  /** Initial route. Components resolve their application name from it, so it is rarely irrelevant. */
  route?: string;
  /** Services to provide, for components reading them through `useInjection`. */
  bindings?: ContainerConfig["bindings"];
}

/**
 * Renders whatever the subject publishes to the left, standing in for `ApplicationPanelSlot`.
 */
function LeftPanelsOutlet(): ReactElement {
  const panels: ReadonlyArray<IEditorPanel> = useEditorPanelsRegistry();

  return (
    <>
      {selectPanelsOnSide(panels, "left").map((panel: IEditorPanel) => (
        <Fragment key={panel.id}>{panel.render()}</Fragment>
      ))}
    </>
  );
}

/**
 * Render a component with the application's test providers.
 */
export function renderWithProviders(ui: ReactNode, { route = "/", bindings = [] }: IRenderOptions = {}): RenderResult {
  const config: ContainerConfig = {
    bindings: [NotificationsService, SettingsService, ...bindings],
    plugins: [new EventsPlugin()],
  };

  function TestRouter({ children }: PropsWithChildren): ReactElement {
    return <MemoryRouter initialEntries={[route]}>{children}</MemoryRouter>;
  }

  function Wrapper({ children }: PropsWithChildren): ReactElement {
    return (
      <ApplicationProvider router={TestRouter}>
        <ContainerProvider config={config}>
          <EditorBusyProvider>
            <EditorStatusProvider>
              <EditorPanelsProvider>
                {children}
                <LeftPanelsOutlet />
              </EditorPanelsProvider>
            </EditorStatusProvider>
          </EditorBusyProvider>
        </ContainerProvider>
      </ApplicationProvider>
    );
  }

  return render(<>{ui}</>, { wrapper: Wrapper });
}
