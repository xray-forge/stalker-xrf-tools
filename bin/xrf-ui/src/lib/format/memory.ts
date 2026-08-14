const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB"] as const;

export function bytesToMegabytes(bytes: number): number {
  return bytes / 1024 / 1024;
}

/**
 * Formats a byte count with an adaptive binary unit.
 *
 * @param bytes - Byte count to format. Negative values are treated as zero.
 * @returns The byte count formatted with an adaptive binary unit.
 */
export function formatBytes(bytes: number): string {
  const safeBytes: number = Math.max(0, bytes);

  if (safeBytes < 1024) {
    return `${safeBytes} B`;
  }

  const unitIndex: number = Math.min(Math.floor(Math.log(safeBytes) / Math.log(1024)), BYTE_UNITS.length - 1);
  const value: number = safeBytes / 1024 ** unitIndex;
  const precision: number = value >= 100 ? 0 : value >= 10 ? 1 : 2;

  return `${Number(value.toFixed(precision))} ${BYTE_UNITS[unitIndex]}`;
}
