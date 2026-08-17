import { Nullable } from "@/lib/types/general";

/**
 * Targets whose own menu is the only clipboard UI they have. Matched by attribute rather than by the
 * `isContentEditable` flag so an ancestor editable region counts and jsdom can exercise it.
 */
const EDITABLE_SELECTOR: string = "input, textarea, [contenteditable]:not([contenteditable='false'])";

/**
 * Checks whether a context menu request came from an editable field.
 *
 * @param target - Target of the context menu event.
 * @returns Whether the target sits in an editable field.
 */
function isEditableTarget(target: Nullable<EventTarget>): boolean {
  return Boolean(target instanceof Element && target.closest(EDITABLE_SELECTOR));
}

/**
 * Suppresses the webview's own context menu so the application can own that gesture.
 *
 * The webview only shows its menu - reload, save as, inspect - when the page lets the event through,
 * and tauri exposes no configuration switch for it, so preventing the event is the whole mechanism.
 * Devtools stay reachable in development builds through the browser accelerator keys.
 *
 * Listens on the document, which means element handlers see the event first. One that opens its own
 * menu must still prevent the default itself: stopping propagation here would hand the gesture back to
 * the webview.
 *
 * @returns Disposer that restores the webview menu.
 */
export function suppressNativeContextMenu(): () => void {
  function onContextMenu(event: MouseEvent): void {
    if (!isEditableTarget(event.target)) {
      event.preventDefault();
    }
  }

  document.addEventListener("contextmenu", onContextMenu);

  return () => document.removeEventListener("contextmenu", onContextMenu);
}
