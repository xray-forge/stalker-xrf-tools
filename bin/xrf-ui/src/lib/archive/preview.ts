import { IArchiveFileDescriptor, IArchiveReadPolicy } from "@/lib/archive/types";

export type ArchivePreviewSupport =
  | { kind: "supported" }
  | { kind: "image" }
  | { kind: "unsupported-extension"; extension: string }
  | { kind: "compressed" }
  | { kind: "too-large"; maximumSize: number };

/**
 * Whether the backend will decode this file into a picture rather than read it as text.
 *
 * Both lists come from the project's own read policy, so the frontend never has to keep its own copy of
 * what the backend is willing to do.
 */
export function isArchiveImage(descriptor: IArchiveFileDescriptor, policy: IArchiveReadPolicy): boolean {
  const extension: string = descriptor.extension.toLowerCase();

  return policy.imageExtensions.some((candidate: string) => candidate.toLowerCase() === extension);
}

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
  // Images are decoded rather than read as text, so they answer to their own limit and - unlike text -
  // do not care whether the entry was stored compressed. Decompression happens on the way out anyway.
  if (isArchiveImage(descriptor, policy)) {
    return descriptor.sizeReal > policy.maximumImageSize
      ? { kind: "too-large", maximumSize: policy.maximumImageSize }
      : { kind: "image" };
  }

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
