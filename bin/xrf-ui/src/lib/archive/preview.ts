import { IArchiveFileDescriptor, IArchiveReadPolicy } from "@/lib/archive/types";

export type ArchivePreviewSupport =
  | { kind: "supported" }
  | { kind: "unsupported-extension"; extension: string }
  | { kind: "compressed" }
  | { kind: "too-large"; maximumSize: number };

/**
 * Determine whether the backend can provide a text preview for an archive file.
 *
 * @param descriptor - Archive file metadata used to validate type, size, and compression state.
 * @param policy - Backend-provided archive read capabilities.
 * @returns A discriminated result describing preview support or the reason it is unavailable.
 */
export function getArchivePreviewSupport(
  descriptor: IArchiveFileDescriptor,
  policy: IArchiveReadPolicy
): ArchivePreviewSupport {
  if (!policy.extensions.some((candidate: string) => candidate === descriptor.extension)) {
    return { kind: "unsupported-extension", extension: descriptor.extension };
  }

  if (descriptor.sizeReal > policy.maximumSize) {
    return { kind: "too-large", maximumSize: policy.maximumSize };
  }

  if (!policy.supportsCompressedFiles && descriptor.sizeReal !== descriptor.sizeCompressed) {
    return { kind: "compressed" };
  }

  return { kind: "supported" };
}
