import { ArchiveAudioPreview, ArchiveImagePreview } from "@/lib/xrf/bindings/xrf-app";
import { ArchiveExtractFolderResult, ArchiveFileDescriptor, ProjectReadResult } from "@/lib/xrf/bindings/xrf-archive";

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
  | { kind: "extract-folder"; result: ArchiveExtractFolderResult };
