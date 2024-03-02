import { GlobalStyles, ThemeProvider } from "@mui/material";
import { default as CssBaseline } from "@mui/material/CssBaseline";
import { useManager } from "dreamstate";
import { ReactNode } from "react";

import { IThemeContext, ThemeManager } from "@/store/theme";

interface IApplicationProviderProps {
  themeContext?: IThemeContext;
  children: ReactNode;
}
export function ApplicationProvider({
  themeContext: { theme } = useManager(ThemeManager),
  children,
}: IApplicationProviderProps) {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <GlobalStyles
        styles={{
          "html, body, #root": {
            width: "100%",
            height: "100%",
          },
        }}
      />
      {children}
    </ThemeProvider>
  );
}
