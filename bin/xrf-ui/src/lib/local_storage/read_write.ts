import { Optional } from "@/core/types/general";

/**
 * @param key - storage key
 * @returns raw value from local storage
 */
export function getLocalStorageValue(key: string): Optional<string> {
  return localStorage.getItem(key);
}

export function setLocalStorageValue(key: string, value: Optional<string>): void {
  if (value === null) {
    localStorage.removeItem(key);
  } else {
    localStorage.setItem(key, value);
  }
}

/**
 * @param key - storage key
 * @returns JSON parsed value from local storage
 */
export function parseLocalStorageValue<T>(key: string): Optional<T> {
  const raw: Optional<string> = localStorage.getItem(key) ?? null;

  return raw === null ? null : JSON.parse(raw);
}
