import { Box } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchiveFileHeader } from "@/applications/archive-editor/components/editor/preview/ArchiveFileHeader";
import { ArchiveFolderContent } from "@/applications/archive-editor/components/editor/preview/ArchiveFolderContent";
import { ArchiveImagePreview } from "@/applications/archive-editor/components/editor/preview/ArchiveImagePreview";
import { ArchivePreviewError } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewError";
import { ArchivePreviewState } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewState";
import { ArchiveTextPreview } from "@/applications/archive-editor/components/editor/preview/ArchiveTextPreview";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { Nullable } from "@/core/types/general";
import {
  ArchivePreviewSupport,
  getArchivePreviewSupport,
  IArchiveFileDescriptor,
  IArchivesProject,
  TArchiveContent,
  TArchiveSelection,
} from "@/lib/archive";
import { Loadable } from "@/lib/loadable";
import { formatBytes } from "@/lib/size";

export function ArchivesFileContent(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const selection: TArchiveSelection = archivesService.selection;
  const project: Nullable<IArchivesProject> = archivesService.project.value;
  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;

  const onGetUnsupportedDescription = useCallback(
    (support: Exclude<ArchivePreviewSupport, { kind: "supported" } | { kind: "image" }>): string => {
      switch (support.kind) {
        case "unsupported-extension":
          return support.extension
            ? `.${support.extension} files can be inspected in Details, ` +
                "but this file type does not have a text preview."
            : "Files without an extension can be inspected in Details, but do not have a text preview.";
        case "too-large":
          return (
            `This file exceeds the ${formatBytes(support.maximumSize)} preview limit. ` +
            "Its archive metadata is still available in Details."
          );
      }
    },
    []
  );

  // A directory selection is a different kind of thing, not a file that happens to be missing.
  if (selection.kind === "directory") {
    return <ArchiveFolderContent path={selection.path} />;
  }

  const descriptor: Nullable<IArchiveFileDescriptor> = selection.kind === "file" ? selection.descriptor : null;

  if (!descriptor || !project) {
    return (
      <ArchivePreviewState
        title={"Select a file to preview"}
        description={
          project
            ? `Supported text files up to ${formatBytes(project.readPolicy.maximumSize)} can be displayed.`
            : "Supported text files can be displayed."
        }
      />
    );
  }

  const support: ArchivePreviewSupport = getArchivePreviewSupport(descriptor, project.readPolicy);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0, minHeight: 0 }}>
      <ArchiveFileHeader descriptor={descriptor} />

      <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>
        {support.kind === "image" ? (
          <ArchiveImagePreview />
        ) : support.kind !== "supported" ? (
          <ArchivePreviewState title={"Preview unavailable"} description={onGetUnsupportedDescription(support)} />
        ) : content.isLoading ? (
          <DelayedProgress />
        ) : content.error ? (
          <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />
        ) : content.value?.kind === "text" ? (
          <ArchiveTextPreview file={content.value.result} />
        ) : (
          <ArchivePreviewState
            title={"Preview unavailable"}
            description={"The selected file did not return any content."}
          />
        )}
      </Box>
    </Box>
  );
}
