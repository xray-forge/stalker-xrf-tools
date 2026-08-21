import { VisualSource } from "@/core/bindings/types/xrf-app";

/**
 * Human readable name of where a visual came from.
 *
 * Switched on `kind` rather than reading a field the one current variant happens to have, so adding an
 * archive entry source is a compile error here instead of silently reading `undefined`.
 *
 * @param source - The visual source to describe.
 * @returns A label identifying the source.
 */
export function describeVisualSource(source: VisualSource): string {
  switch (source.kind) {
    case "file":
      return source.path;
  }
}
