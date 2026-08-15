import { EmotionCache } from "@emotion/cache";
import { CacheProvider } from "@emotion/react";
import { GlobalStyles } from "@mui/material";
import { default as CssBaseline } from "@mui/material/CssBaseline";
import { Theme, ThemeProvider } from "@mui/material/styles";
import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ComponentType, PropsWithChildren, ReactElement, ReactNode, useMemo } from "react";
import { BrowserRouter } from "react-router-dom";

import { ErrorCaptureService, NotificationsService } from "@/core/notifications/services";
import { ProjectService } from "@/core/settings/services/project";
import { SettingsService } from "@/core/settings/services/settings";
import { createApplicationStyleCache, createApplicationTheme } from "@/core/theme";
import { useContainerGeneration } from "@/lib/react";

// todo: Verify and see whether we need better HMR support inside of wirestate.
const ROOT_CONTAINER_CONFIG: ContainerConfig = {
  bindings: [ProjectService, SettingsService, NotificationsService, ErrorCaptureService],
  plugins: [new EventsPlugin()],
};

interface IApplicationProviderProps {
  router?: ComponentType<PropsWithChildren>;
  children: ReactNode;
}

export function ApplicationProvider({
  router: Router = BrowserRouter,
  children,
}: IApplicationProviderProps): ReactElement {
  const theme: Theme = useMemo(() => createApplicationTheme(), []);
  const cache: EmotionCache = useMemo(() => createApplicationStyleCache(), []);

  const generation: number = useContainerGeneration(ROOT_CONTAINER_CONFIG);

  return (
    <ContainerProvider key={generation} config={ROOT_CONTAINER_CONFIG}>
      <CacheProvider value={cache}>
        <ThemeProvider
          defaultMode={"dark"}
          disableTransitionOnChange={true}
          modeStorageKey={"theme"}
          noSsr={true}
          theme={theme}
        >
          <CssBaseline enableColorScheme={true} />

          <GlobalStyles
            styles={{
              "html, body, #root": {
                width: "100%",
                height: "100%",
                minHeight: 360,
                minWidth: 400,
              },
            }}
          />

          <Router>{children}</Router>
        </ThemeProvider>
      </CacheProvider>
    </ContainerProvider>
  );
}
