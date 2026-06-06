import { Optional } from "@/core/types/general";

/**
 * @param key - storage key
 * @returns raw value from local storage
 */
export function getLocalStorageValue(key: string): Optional<string> {
  return window.localStorage ? window.localStorage.getItem(key) : null;
}

export function setLocalStorageValue(key: string, value: Optional<string>): void {
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
 * @param key - storage key
 * @returns JSON parsed value from local storage
 */
export function parseLocalStorageValue<T>(key: string): Optional<T> {
  if (!window.localStorage) {
    return null;
  }

  const raw: Optional<string> = window.localStorage.getItem(key) ?? null;

  return raw === null ? null : JSON.parse(raw);
}
