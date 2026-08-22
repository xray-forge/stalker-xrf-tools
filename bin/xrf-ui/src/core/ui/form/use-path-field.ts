import { DialogFilter } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useRef, useState } from "react";

import { EApplicationId } from "@/core/routing/application";
import { usePathState } from "@/core/ui/form/file-picker/use-path-state";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { Nullable } from "@/lib/types/general";

const STORAGE_PREFIX: string = "xrf.form.";
const VALIDATE_DEBOUNCE_MS: number = 250;

export interface IPathFieldOptions {
  /** The application that owns this field. */
  application: EApplicationId;
  /** Names the field inside its own form, for example `source`. Only unique within the application. */
  id: string;
  title?: string;
  filters?: Nullable<Array<DialogFilter>>;
  isDirectory?: boolean;
  isSave?: boolean;
  isDisabled?: boolean;
  isRequired?: boolean;
  /** Produces a first guess when nothing has been remembered yet, usually from the project root. */
  seed?: () => Promise<Nullable<string>>;
}

export interface IPathField {
  value: Nullable<string>;
  error: Nullable<string>;
  isValid: boolean;
  select: () => Promise<void>;
  clear: () => void;
  setValue: (value: Nullable<string>) => void;
}

/**
 * Manages a remembered, validated filesystem path for one form field.
 *
 * What is stored is only ever what a caller asked to store: the remembered path is read once, during
 * the first render, and written only by `select`, `clear`, and `setValue`. Nothing about mounting,
 * remounting, or seeding touches storage, so no session can overwrite or erase what an earlier one
 * remembered while its own state is still empty.
 *
 * @param options - Field identity, dialog behavior, and validation options.
 * @param options.application - Application the field belongs to, used to scope persistence.
 * @param options.id - Field name within that application, used for persistence.
 * @param options.title - Path dialog title.
 * @param options.filters - File filters shown by the dialog.
 * @param options.isDirectory - Whether the dialog selects a directory.
 * @param options.isSave - Whether the path may identify a new output file.
 * @param options.isDisabled - Whether selection is disabled.
 * @param options.isRequired - Whether an empty path is invalid.
 * @param options.seed - Async fallback used when no path is stored.
 * @returns The current path state, validation result, and field actions.
 */
export function usePathField({
  application,
  id,
  title,
  filters = null,
  isDirectory = false,
  isSave = false,
  isDisabled = false,
  isRequired = true,
  seed,
}: IPathFieldOptions): IPathField {
  const storageKey: string = `${STORAGE_PREFIX}${application}.${id}`;

  const [value, setPath, selectPath] = usePathState({
    title,
    filters,
    isDirectory,
    isSave,
    isDisabled,
    initial: () => getLocalStorageValue(storageKey),
  });
  const [error, setError] = useState<Nullable<string>>(null);

  const setValue = useCallback(
    (next: Nullable<string>): void => {
      setPath(next);
      setLocalStorageValue(storageKey, next);
    },
    [setPath, storageKey]
  );

  const clear = useCallback(() => setValue(null), [setValue]);

  const select = useCallback(async (): Promise<void> => {
    const picked: Nullable<string> = await selectPath();

    // Cancelling reports null and leaves both the state and what was remembered as they were.
    if (picked !== null) {
      setLocalStorageValue(storageKey, picked);
    }
  }, [selectPath, storageKey]);

  // Seeding happens once, and only when nothing was remembered. A guess is not a choice, so it fills
  // the field without being stored, and every session re-derives it until the user picks a path.
  const seedRef = useRef<Nullable<IPathFieldOptions["seed"]>>(seed);
  const isSeedResolved = useRef<boolean>(false);

  seedRef.current = seed;

  useEffect(() => {
    if (isSeedResolved.current) {
      return;
    }

    isSeedResolved.current = true;

    if (value !== null || !seedRef.current) {
      return;
    }

    seedRef
      .current()
      .then((seeded) => seeded && setPath(seeded))
      .catch(() => setPath(null));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [storageKey]);

  useEffect(() => {
    if (!value || isSave) {
      setError(null);

      return;
    }

    let isCurrent: boolean = true;

    const handle = setTimeout(() => {
      exists(value)
        .then((isPresent) => isCurrent && setError(isPresent ? null : "Path does not exist"))
        // A failed check is not proof of absence, so it is reported as unknown rather than missing.
        .catch(() => isCurrent && setError(null));
    }, VALIDATE_DEBOUNCE_MS);

    return () => {
      isCurrent = false;
      clearTimeout(handle);
    };
  }, [value, isSave]);

  return {
    value,
    error,
    isValid: (!isRequired || Boolean(value)) && !error,
    select,
    clear,
    setValue,
  };
}
