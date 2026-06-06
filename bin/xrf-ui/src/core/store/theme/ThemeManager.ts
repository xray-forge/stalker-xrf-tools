import { PaletteMode, Theme } from "@mui/material";
import { Injectable, OnDeprovision, OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable } from "@wirestate/react-mobx";

import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local_storage";
import { Logger } from "@/lib/logging";
import { createApplicationTheme } from "@/lib/theme";

@Injectable()
export class ThemeManager {
  @Observable()
  public theme: Theme = createApplicationTheme(getLocalStorageValue("theme") === "light" ? "light" : "dark");

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor() {
    makeObservable(this);
  }

  @OnProvision()
  public onProvision(): void {
    this.log.info("Theme provision started:", this.theme.palette.mode);
  }

  @OnDeprovision()
  public onDeprovision(): void {
    this.log.info("Theme provision ended");
  }

  @BoundAction()
  public async toggleTheme(): Promise<void> {
    const nextThemeMode: PaletteMode = this.theme.palette.mode === "light" ? "dark" : "light";
    const nextTheme: Theme = createApplicationTheme(nextThemeMode);

    this.log.info("Toggle theme mode to:", nextThemeMode);

    this.theme = nextTheme;

    setLocalStorageValue("theme", nextThemeMode);
  }
}
