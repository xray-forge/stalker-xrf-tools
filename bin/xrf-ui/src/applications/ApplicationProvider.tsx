import { GlobalStyles, ThemeProvider } from "@mui/material";
import { default as CssBaseline } from "@mui/material/CssBaseline";
import { useInjection } from "@wirestate/react";
import { ReactNode } from "react";

import { ThemeService } from "@/core/store/theme";

interface IApplicationProviderProps {
  children: ReactNode;
}

export function ApplicationProvider({ children }: IApplicationProviderProps) {
  const { theme } = useInjection(ThemeService);

  return (
    <ThemeProvider theme={theme}>
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
