/**
 * Decodes base64 into its source bytes.
 *
 * `atob` yields a binary string, one code unit per byte. Handing that string to a `Blob` directly would
 * encode it as UTF-8 and turn every byte above 0x7f into two, corrupting any container header it is
 * used for - which is most of them.
 *
 * @param base64 - Standard-alphabet base64 returned by backend commands.
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
 * Wraps base64-encoded bytes in a blob.
 *
 * @param base64 - Standard-alphabet base64 returned by backend commands.
 * @param type - MIME type used to identify the blob content.
 * @returns A blob over the decoded bytes.
 */
export function base64ToBlob(base64: string, type: string): Blob {
  return new Blob([base64ToBytes(base64).buffer as ArrayBuffer], { type });
}
