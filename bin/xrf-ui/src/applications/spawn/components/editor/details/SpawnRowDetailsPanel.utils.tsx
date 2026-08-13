import { AnyObject } from "@/lib/types/general";

/** Vectors, tuples and nested objects all have to land as something a person can read in one line. */
export function formatSpawnRowDetailsValue(value: unknown): string {
  if (value === null || value === undefined) {
    return "-";
  }

  if (Array.isArray(value)) {
    return value.length ? value.map(formatSpawnRowDetailsValue).join(", ") : "empty";
  }

  if (typeof value === "object") {
    const entries: Array<[string, unknown]> = Object.entries(value as AnyObject);

    return entries
      .map(([key, nested]: [string, unknown]) => `${key}: ${formatSpawnRowDetailsValue(nested)}`)
      .join(", ");
  }

  return String(value);
}
