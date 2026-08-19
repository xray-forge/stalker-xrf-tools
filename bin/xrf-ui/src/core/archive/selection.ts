import { ArchiveAudioPreview, ArchiveImagePreview } from "@/core/bindings/xrf-app";
import {
  ArchiveExtractDirectoryResult,
  ArchiveFileDescriptor,
  ProjectReadResult,
} from "@/core/bindings/xrf-app-archives";

/**
 * What the explorer currently points at.
 */
export type TArchiveSelection =
  | { kind: "none" }
  | { kind: "file"; descriptor: ArchiveFileDescriptor }
  | { kind: "directory"; path: string };

/**
 * What was loaded for the current selection, whatever form it took.
 */
export type TArchiveContent =
  | { kind: "text"; result: ProjectReadResult }
  | { kind: "image"; preview: ArchiveImagePreview }
  | { kind: "audio"; preview: ArchiveAudioPreview };

/** The last thing written to disk, so the surface that started it can report what happened. */
export type TArchiveOperation =
  | { kind: "extract-file"; destination: string }
  | { kind: "extract-directory"; result: ArchiveExtractDirectoryResult };
