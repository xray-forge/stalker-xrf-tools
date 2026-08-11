import { Injectable, OnDeactivation } from "@wirestate/core";

import { Nullable } from "@/core/types/general";
import { Logger } from "@/lib/logging";

/**
 * Owns the lifetime of every object url an editor hands to the webview.
 *
 * Object urls are not garbage collected: a blob stays alive until its url is revoked, so anything that
 * creates one has to remember to release it. Every place that did this by hand got it wrong in a
 * different way - `blobToImage` gave up and left the revoke commented out, and the icons editor revoked
 * before its replacement existed, leaving the viewer pointed at a dead url.
 */
@Injectable()
export class AssetService {
  public readonly log: Logger = new Logger(this.constructor.name);

  /** Urls held against a caller supplied key, so a new one can displace the old automatically. */
  private readonly keyed: Map<string, string> = new Map();

  /** Urls the caller owns outright and must release itself. */
  private readonly loose: Set<string> = new Set();

  /**
   * Sweep whatever the editor still held on the way out.
   */
  @OnDeactivation()
  public onDeactivation(): void {
    if (this.heldCount) {
      this.log.info("Releasing held object urls:", this.heldCount);
    }

    this.releaseAll();
  }

  /**
   * Create a url the caller is responsible for releasing.
   *
   * Prefer `swap` where there is a natural key - a selection, a sprite - so releasing is not something
   * anyone has to remember.
   */
  public create(blob: Blob): string {
    const url: string = URL.createObjectURL(blob);

    this.loose.add(url);

    return url;
  }

  /**
   * Replace whatever url is held under `key` and return the new one.
   *
   * The new url is created before the old is revoked, which is the ordering that matters: revoking
   * first leaves anything still rendering the old url pointing at nothing for as long as the
   * replacement takes, and permanently if creating it fails.
   */
  public swap(key: string, blob: Blob): string {
    const previous: Nullable<string> = this.keyed.get(key) ?? null;
    const url: string = URL.createObjectURL(blob);

    this.keyed.set(key, url);

    if (previous) {
      URL.revokeObjectURL(previous);
    }

    return url;
  }

  /** Release one url obtained from `create`. Unknown urls are ignored rather than revoked blindly. */
  public release(url: Nullable<string>): void {
    if (url && this.loose.delete(url)) {
      URL.revokeObjectURL(url);
    }
  }

  /** Release whatever is held under a key, if anything. */
  public releaseKey(key: string): void {
    const url: Nullable<string> = this.keyed.get(key) ?? null;

    if (url) {
      this.keyed.delete(key);
      URL.revokeObjectURL(url);
    }
  }

  /** Number of urls still held, so a test can assert nothing was left behind. */
  public get heldCount(): number {
    return this.keyed.size + this.loose.size;
  }

  public releaseAll(): void {
    for (const url of this.keyed.values()) {
      URL.revokeObjectURL(url);
    }

    for (const url of this.loose) {
      URL.revokeObjectURL(url);
    }

    this.keyed.clear();
    this.loose.clear();
  }
}
