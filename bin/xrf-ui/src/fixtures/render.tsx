import { render, RenderResult } from "@testing-library/react";
import { ContainerConfig } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactElement, ReactNode } from "react";
import { MemoryRouter } from "react-router-dom";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { EditorStatusProvider } from "@/core/components/shell/EditorStatusContext";

export interface IRenderOptions {
  /** Initial route. Components resolve their tool name from it, so it is rarely irrelevant. */
  route?: string;
  /** Services to provide, for components reading them through `useInjection`. */
  bindings?: ContainerConfig["bindings"];
}

/** Render a component with the application's test providers. */
export function renderWithProviders(ui: ReactNode, { route = "/", bindings = [] }: IRenderOptions = {}): RenderResult {
  function Wrapper({ children }: { children: ReactNode }): ReactElement {
    return (
      <MemoryRouter initialEntries={[route]}>
        <ApplicationProvider>
          <ContainerProvider config={{ bindings }}>
            <EditorStatusProvider>{children}</EditorStatusProvider>
          </ContainerProvider>
        </ApplicationProvider>
      </MemoryRouter>
    );
  }

  return render(<>{ui}</>, { wrapper: Wrapper });
}
