import { DialogFilter, open, save } from "@tauri-apps/plugin-dialog";
import { Dispatch, SetStateAction, useCallback, useState } from "react";

import { Nullable } from "@/core/types/general";

export interface IPathStateOptions {
  title?: string;
  filters?: Nullable<Array<DialogFilter>>;
  isDisabled?: boolean;
  isDirectory?: boolean;
  /** Ask where to write instead of what to read. Pack screens choose an output file this way. */
  isSave?: boolean;
}

export type TPathState = [Nullable<string>, Dispatch<SetStateAction<Nullable<string>>>, () => Promise<void>];

/**
 * Hold a picked path and the action that fills it.
 *
 * Barely more than `useState` plus a configured dialog call, and that is the point: the two guards it
 * owns - refusing to open while disabled, and leaving the current value alone when the user cancels.
 */
export function usePathState({
  title = "Provide path",
  filters = null,
  isDisabled = false,
  isDirectory = false,
  isSave = false,
}: IPathStateOptions = {}): TPathState {
  const [pathState, setPathState] = useState<Nullable<string>>(null);

  // Filters are declared inline by callers, so their identity changes every render. Comparing by
  // content keeps the callback stable without asking every caller to memoise.
  const filtersKey: string = JSON.stringify(filters);

  const onSelectPath = useCallback(async () => {
    if (isDisabled) {
      return;
    }

    const pathResponse: Nullable<string> = isSave
      ? await save({ title, filters: filters ? filters : undefined })
      : await open({ title, filters: filters ? filters : undefined, directory: isDirectory });

    // Cancelling resolves null. Keeping the previous value is deliberate: replacing a good path with
    // nothing because someone opened the dialog and thought better of it is never what was wanted.
    if (pathResponse) {
      setPathState(pathResponse);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [title, isDirectory, isDisabled, isSave, filtersKey]);

  return [pathState, setPathState, onSelectPath];
}
