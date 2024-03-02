/**
 * Generic console logger with prefix and colored display.
 */
export class Logger {
  public static readonly PREFIX_COLOR: string = "color: #bada53";

  public prefix: string;
  public isEnabled: boolean;

  public constructor(prefix: string, isEnabled: boolean = true) {
    this.prefix = prefix;
    this.isEnabled = isEnabled;
  }

  public info(...messages: Array<unknown>): void {
    window.console.info(`%c [${this.prefix}]`, Logger.PREFIX_COLOR, ...messages);
  }
}
