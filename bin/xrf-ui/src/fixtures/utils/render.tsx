import { render, RenderResult } from "@testing-library/react";
import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { Fragment, PropsWithChildren, ReactElement, ReactNode } from "react";
import { MemoryRouter } from "react-router-dom";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { EditorBusyProvider } from "@/core/components/shell/EditorBusyContext";
import {
  EditorPanelsProvider,
  IEditorPanel,
  selectPanelsOnSide,
  useEditorPanelsRegistry,
} from "@/core/components/shell/EditorPanelsContext";
import { EditorStatusProvider } from "@/core/components/shell/EditorStatusContext";
import { NotificationsService } from "@/core/store/notifications";
import { SettingsService } from "@/core/store/settings";

export interface IRenderOptions {
  /** Initial route. Components resolve their application name from it, so it is rarely irrelevant. */
  route?: string;
  /** Services to provide, for components reading them through `useInjection`. */
  bindings?: ContainerConfig["bindings"];
}

/**
 * Renders whatever the subject publishes to the left, the way the shell's rail slot does.
 */
function LeftPanelsOutlet(): ReactElement {
  const panels: Array<IEditorPanel> = useEditorPanelsRegistry();

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

  function Wrapper({ children }: PropsWithChildren): ReactElement {
    return (
      <MemoryRouter initialEntries={[route]}>
        <ApplicationProvider>
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
      </MemoryRouter>
    );
  }

  return render(<>{ui}</>, { wrapper: Wrapper });
}
