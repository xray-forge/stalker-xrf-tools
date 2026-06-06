import { GlobalStyles } from "@mui/material";
import { default as CssBaseline } from "@mui/material/CssBaseline";
import { Theme, ThemeProvider } from "@mui/material/styles";
import { ReactNode, useMemo } from "react";

import { createApplicationTheme } from "@/lib/theme";

interface IApplicationProviderProps {
  children: ReactNode;
}

export function ApplicationProvider({ children }: IApplicationProviderProps) {
  const theme: Theme = useMemo(() => createApplicationTheme(), []);

  return (
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
  );
}
