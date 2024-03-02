import { createProvider, ScopeProvider } from "dreamstate";
import { ReactElement } from "react";

import { ApplicationProvider } from "@/ApplicationProvider";
import { ApplicationRouter } from "@/ApplicationRouter";
import { ThemeManager } from "@/core/store/theme";

const GlobalProvider = createProvider([ThemeManager]);

export function Application(): ReactElement {
  return (
    <ScopeProvider>
      <GlobalProvider>
        <ApplicationProvider>
          <ApplicationRouter />
        </ApplicationProvider>
      </GlobalProvider>
    </ScopeProvider>
  );
}
