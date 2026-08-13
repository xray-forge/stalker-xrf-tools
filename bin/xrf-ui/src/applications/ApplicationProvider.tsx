import { EmotionCache } from "@emotion/cache";
import { CacheProvider } from "@emotion/react";
import { GlobalStyles } from "@mui/material";
import { default as CssBaseline } from "@mui/material/CssBaseline";
import { Theme, ThemeProvider } from "@mui/material/styles";
import { ContainerConfig, EventsPlugin } from "@wirestate/core";
import { ContainerProvider } from "@wirestate/react";
import { ReactNode, useMemo } from "react";

import { ErrorCaptureService, NotificationsService } from "@/core/services/notifications";
import { ProjectService } from "@/core/services/project";
import { SettingsService } from "@/core/services/settings";
import { createApplicationStyleCache, createApplicationTheme } from "@/lib/theme";

interface IApplicationProviderProps {
  children: ReactNode;
}

export function ApplicationProvider({ children }: IApplicationProviderProps) {
  const theme: Theme = useMemo(() => createApplicationTheme(), []);
  const cache: EmotionCache = useMemo(() => createApplicationStyleCache(), []);

  const config: ContainerConfig = useMemo(
    () => ({
      bindings: [ProjectService, SettingsService, NotificationsService, ErrorCaptureService],
      plugins: [new EventsPlugin()],
    }),
    []
  );

  return (
    <ContainerProvider config={config}>
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

          {children}
        </ThemeProvider>
      </CacheProvider>
    </ContainerProvider>
  );
}
