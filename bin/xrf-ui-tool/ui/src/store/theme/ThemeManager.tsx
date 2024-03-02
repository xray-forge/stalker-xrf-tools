import { PaletteMode, Theme } from "@mui/material";
import { createTheme } from "@mui/material/styles";
import { ContextManager } from "dreamstate";

import { Logger } from "@/lib/logging";

export interface IThemeContext {
  themeActions: {
    toggleTheme(): void;
  };
  theme: Theme;
}

/**
 * Context manager related to theme management.
 * It is responsible for theming, styling and generation configuration.
 */
export class ThemeManager extends ContextManager<IThemeContext> {
  public readonly context: IThemeContext = {
    themeActions: {
      toggleTheme: this.toggleTheme.bind(this),
    },
    theme: createTheme({
      palette: {
        mode: "dark",
      },
    }),
  };

  public log: Logger = new Logger("thm");

  /**
   * Send generic message on manager provision start and subscribe to events.
   */
  public onProvisionStarted(): void {
    const { theme } = this.context;

    this.log.info("Theme provision started:", theme.palette.mode);
  }

  /**
   * Unsubscribe from events after provision end.
   */
  public onProvisionEnded(): void {
    this.log.info("Theme provision ended.");
  }

  /**
   * Toggle application theme mode and save it into local storage.
   * Apply it to document body.
   */
  public async toggleTheme(): Promise<void> {
    const { theme } = this.context;

    const nextThemeMode: PaletteMode = theme.palette.mode === "light" ? "dark" : "light";
    const nextTheme: Theme = createTheme({ palette: { mode: nextThemeMode } });

    this.log.info("Toggle theme mode to:", nextThemeMode);

    this.setContext({ theme: nextTheme });
  }
}
