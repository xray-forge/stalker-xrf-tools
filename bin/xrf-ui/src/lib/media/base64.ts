/**
 * Widen base64 into the bytes it stands for.
 *
 * `atob` yields a binary string, one code unit per byte. Handing that string to a `Blob` directly would
 * encode it as UTF-8 and turn every byte above 0x7f into two, corrupting any container header it is
 * used for - which is most of them.
 *
 * @param base64 - Standard alphabet base64, as the backend commands return it.
 * @returns The decoded bytes, one per source byte.
 */
export function base64ToBytes(base64: string): Uint8Array {
  const binary: string = atob(base64);
  const bytes: Uint8Array = new Uint8Array(binary.length);

  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }

  return bytes;
}

/**
 * Wrap base64 encoded bytes as a blob, for anything that needs a url rather than the bytes.
 *
 * @param base64 - Standard alphabet base64, as the backend commands return it.
 * @param type - Mime type to tag the blob with, so a consumer can decode it.
 * @returns A blob over the decoded bytes.
 */
export function base64ToBlob(base64: string, type: string): Blob {
  return new Blob([base64ToBytes(base64).buffer as ArrayBuffer], { type });
}
