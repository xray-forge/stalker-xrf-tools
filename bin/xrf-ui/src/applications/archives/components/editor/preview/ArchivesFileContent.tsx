import { Box } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchiveAudioPreview } from "@/applications/archives/components/editor/preview/ArchiveAudioPreview";
import { ArchiveCodePreview } from "@/applications/archives/components/editor/preview/ArchiveCodePreview";
import { ArchiveFileHeader } from "@/applications/archives/components/editor/preview/ArchiveFileHeader";
import { ArchiveFolderContent } from "@/applications/archives/components/editor/preview/ArchiveFolderContent";
import { ArchiveImagePreview } from "@/applications/archives/components/editor/preview/ArchiveImagePreview";
import { ArchivePreviewError } from "@/applications/archives/components/editor/preview/ArchivePreviewError";
import { ArchivesService } from "@/applications/archives/store/archives";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { EmptyState } from "@/core/components/layout/EmptyState";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { Loadable } from "@/lib/loadable";
import { formatBytes } from "@/lib/size";
import { ArchivePreviewSupport, getArchivePreviewSupport, TArchiveContent, TArchiveSelection } from "@/lib/xrf/archive";
import { ArchiveFileDescriptor, ArchiveProject } from "@/lib/xrf/bindings/xrf-archive";

// Everything that renders its own preview leaves this union; what is left is a reason to explain.
type TUnsupported = Exclude<ArchivePreviewSupport, { kind: "supported" } | { kind: "image" } | { kind: "audio" }>;

export function ArchivesFileContent({
  "data-testid": dataTestId = "archives-file-content",
  id = "archives-file-content",
  className,
}: BaseComponentProps): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const selection: TArchiveSelection = archivesService.selection;
  const project: Nullable<ArchiveProject> = archivesService.project.value;
  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;

  const onGetUnsupportedDescription = useCallback((support: TUnsupported): string => {
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
  }, []);

  // A directory selection is a different kind of thing, not a file that happens to be missing.
  if (selection.kind === "directory") {
    return <ArchiveFolderContent path={selection.path} />;
  }

  const descriptor: Nullable<ArchiveFileDescriptor> = selection.kind === "file" ? selection.descriptor : null;

  if (!descriptor || !project) {
    return (
      <EmptyState
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
    <Box
      data-testid={dataTestId}
      id={id}
      className={className}
      sx={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0, minHeight: 0 }}
    >
      <ArchiveFileHeader descriptor={descriptor} />

      <Box
        sx={{
          display: "flex",
          flexGrow: 1,
          minWidth: 0,
          minHeight: 0,
          overflow: "hidden",
        }}
      >
        {support.kind === "image" ? (
          <ArchiveImagePreview />
        ) : support.kind === "audio" ? (
          <ArchiveAudioPreview />
        ) : support.kind !== "supported" ? (
          <EmptyState title={"Preview unavailable"} description={onGetUnsupportedDescription(support)} />
        ) : content.isLoading ? (
          <DelayedProgress />
        ) : content.error ? (
          <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />
        ) : content.value?.kind === "text" ? (
          <ArchiveCodePreview file={content.value.result} />
        ) : (
          <EmptyState title={"Preview unavailable"} description={"The selected file did not return any content."} />
        )}
      </Box>
    </Box>
  );
}
