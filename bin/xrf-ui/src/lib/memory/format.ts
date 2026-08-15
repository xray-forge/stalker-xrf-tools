import { BYTES_PER_KILOBYTE } from "@/lib/memory/size";

const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB"] as const;

/**
 * Formats a byte count with an adaptive binary unit.
 *
 * @param bytes - Byte count to format. Negative values are treated as zero.
 * @returns The byte count formatted with an adaptive binary unit.
 */
export function formatBytes(bytes: number): string {
  const safeBytes: number = Math.max(0, bytes);

  if (safeBytes < BYTES_PER_KILOBYTE) {
    return `${safeBytes} B`;
  }

  const unitIndex: number = Math.min(
    Math.floor(Math.log(safeBytes) / Math.log(BYTES_PER_KILOBYTE)),
    BYTE_UNITS.length - 1
  );
  const value: number = safeBytes / BYTES_PER_KILOBYTE ** unitIndex;
  const precision: number = value >= 100 ? 0 : value >= 10 ? 1 : 2;

  return `${Number(value.toFixed(precision))} ${BYTE_UNITS[unitIndex]}`;
}
