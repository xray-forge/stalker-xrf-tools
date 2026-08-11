import { IArchiveFileDescriptor } from "@/lib/archive/types";

const MAX_PREVIEW_SIZE: number = 10 * 1024 * 1024;
const PREVIEW_EXTENSIONS = ["ltx", "script"] as const;

export type ArchivePreviewSupport =
  | { kind: "supported" }
  | { kind: "unsupported-extension"; extension: string }
  | { kind: "compressed" }
  | { kind: "too-large"; maximumSize: number };

/**
 * Determine whether the backend can provide a text preview for an archive file.
 *
 * @param descriptor - Archive file metadata used to validate type, size, and compression state.
 * @returns A discriminated result describing preview support or the reason it is unavailable.
 */
export function getArchivePreviewSupport(descriptor: IArchiveFileDescriptor): ArchivePreviewSupport {
  if (!PREVIEW_EXTENSIONS.some((candidate: string) => candidate === descriptor.extension)) {
    return { kind: "unsupported-extension", extension: descriptor.extension };
  }

  if (descriptor.sizeReal > MAX_PREVIEW_SIZE) {
    return { kind: "too-large", maximumSize: MAX_PREVIEW_SIZE };
  }

  if (descriptor.sizeReal !== descriptor.sizeCompressed) {
    return { kind: "compressed" };
  }

  return { kind: "supported" };
}
