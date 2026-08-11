import { Box } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchiveFileHeader } from "@/applications/archive-editor/components/editor/preview/ArchiveFileHeader";
import { ArchiveFolderContent } from "@/applications/archive-editor/components/editor/preview/ArchiveFolderContent";
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
} from "@/lib/archive";
import { formatBytes } from "@/lib/size";

export function ArchivesFileContent(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const descriptor: Nullable<IArchiveFileDescriptor> = archivesService.fileDescriptor;
  const project: Nullable<IArchivesProject> = archivesService.project.value;
  const directoryPath: Nullable<string> = archivesService.directoryPath;

  const onGetUnsupportedDescription = useCallback(
    (support: Exclude<ArchivePreviewSupport, { kind: "supported" }>): string => {
      switch (support.kind) {
        case "unsupported-extension":
          return support.extension
            ? `.${support.extension} files can be inspected in Details, ` +
                "but this file type does not have a text preview."
            : "Files without an extension can be inspected in Details, but do not have a text preview.";
        case "compressed":
          return (
            "This file is compressed. Its archive metadata is available in Details, " +
            "but compressed previews are not supported yet."
          );
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
  if (directoryPath !== null) {
    return <ArchiveFolderContent path={directoryPath} />;
  }

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
        {support.kind !== "supported" ? (
          <ArchivePreviewState title={"Preview unavailable"} description={onGetUnsupportedDescription(support)} />
        ) : archivesService.file.isLoading ? (
          <DelayedProgress />
        ) : archivesService.file.error ? (
          <ArchivePreviewError error={archivesService.file.error} onRetry={archivesService.retrySelectedFile} />
        ) : archivesService.file.value ? (
          <ArchiveTextPreview file={archivesService.file.value} />
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
