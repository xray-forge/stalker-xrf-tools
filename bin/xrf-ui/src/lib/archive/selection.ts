import {
  IArchiveAudioPreview,
  IArchiveFileDescriptor,
  IArchiveFileReadResult,
  IArchiveFolderExtractResult,
  IArchiveImagePreview,
} from "@/lib/archive/types";

/**
 * What the explorer currently points at.
 */
export type TArchiveSelection =
  | { kind: "none" }
  | { kind: "file"; descriptor: IArchiveFileDescriptor }
  | { kind: "directory"; path: string };

/**
 * What was loaded for the current selection, whatever form it took.
 */
export type TArchiveContent =
  | { kind: "text"; result: IArchiveFileReadResult }
  | { kind: "image"; preview: IArchiveImagePreview }
  | { kind: "audio"; preview: IArchiveAudioPreview };

/** The last thing written to disk, so the surface that started it can report what happened. */
export type TArchiveOperation =
  | { kind: "extract-file"; destination: string }
  | { kind: "extract-folder"; result: IArchiveFolderExtractResult };
