import { Nullable } from "@/lib/types/general";

/**
 * Reads a raw local storage value.
 *
 * @param key - Storage key to read.
 * @returns The stored value, or `null` when unavailable or absent.
 */
export function getLocalStorageValue(key: string): Nullable<string> {
  return window.localStorage ? window.localStorage.getItem(key) : null;
}

export function setLocalStorageValue(key: string, value: Nullable<string>): void {
  if (!window.localStorage) {
    return;
  }

  if (value === null) {
    window.localStorage.removeItem(key);
  } else {
    window.localStorage.setItem(key, value);
  }
}

/**
 * Parses a JSON value from local storage.
 *
 * @param key - Storage key to read.
 * @returns The parsed value, or `null` when storage is unavailable or the key is absent.
 */
export function parseLocalStorageValue<T>(key: string): Nullable<T> {
  if (!window.localStorage) {
    return null;
  }

  const raw: Nullable<string> = window.localStorage.getItem(key) ?? null;

  return raw === null ? null : JSON.parse(raw);
}
