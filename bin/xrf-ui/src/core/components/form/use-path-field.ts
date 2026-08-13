import { DialogFilter } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useRef, useState } from "react";

import { usePathState } from "@/core/components/form/file-picker/use-path-state";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { Nullable } from "@/lib/types/general";

const STORAGE_PREFIX: string = "xrf.form.";
const VALIDATE_DEBOUNCE_MS: number = 250;

export interface IPathFieldOptions {
  /**
   * Stable identifier, for example `spawn.unpack.source`.
   *
   * Deliberately not derived from the route: routes and files have been renamed repeatedly here, and a
   * key built from them discards the user's remembered paths silently every time.
   */
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

export function getPathFieldStorageKey(id: string): string {
  return `${STORAGE_PREFIX}${id}`;
}

/**
 * A remembered, validated filesystem path for one form field.
 */
export function usePathField({
  id,
  title,
  filters = null,
  isDirectory = false,
  isSave = false,
  isDisabled = false,
  isRequired = true,
  seed,
}: IPathFieldOptions): IPathField {
  const [value, setValue, select] = usePathState({ title, filters, isDirectory, isSave, isDisabled });
  const [error, setError] = useState<Nullable<string>>(null);

  const isSeeded = useRef<boolean>(false);
  const storageKey: string = getPathFieldStorageKey(id);

  const clear = useCallback(() => setValue(null), [setValue]);

  // Restore, or fall back to the seed exactly once.
  useEffect(() => {
    if (isSeeded.current) {
      return;
    }

    isSeeded.current = true;

    const stored: Nullable<string> = getLocalStorageValue(storageKey);

    if (stored) {
      setValue(stored);

      return;
    }

    if (seed) {
      seed()
        .then((seeded) => seeded && setValue(seeded))
        .catch(() => setValue(null));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [storageKey]);

  useEffect(() => {
    setLocalStorageValue(storageKey, value);
  }, [storageKey, value]);

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
