import { Box } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { ArchiveAudioPreview } from "@/applications/archives-explorer/components/editor/preview/ArchiveAudioPreview";
import { ArchiveCodePreview } from "@/applications/archives-explorer/components/editor/preview/ArchiveCodePreview";
import { ArchiveDirectoryContent } from "@/applications/archives-explorer/components/editor/preview/ArchiveDirectoryContent";
import { ArchiveFileHeader } from "@/applications/archives-explorer/components/editor/preview/ArchiveFileHeader";
import { ArchiveImagePreview } from "@/applications/archives-explorer/components/editor/preview/ArchiveImagePreview";
import { ArchivePreviewError } from "@/applications/archives-explorer/components/editor/preview/ArchivePreviewError";
import { ArchivesService } from "@/applications/archives-explorer/services/archives";
import { ArchivePreviewSupport, getArchivePreviewSupport, TArchiveContent, TArchiveSelection } from "@/core/archive";
import { ArchiveFileDescriptor, ArchiveProject } from "@/core/bindings/types/xrf-archive";
import { DelayedProgress } from "@/core/ui/layout/DelayedProgress";
import { EmptyState } from "@/core/ui/layout/EmptyState";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { Loadable } from "@/lib/loadable";
import { formatBytes } from "@/lib/memory/format";
import { Nullable } from "@/lib/types/general";

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
    return <ArchiveDirectoryContent path={selection.path} />;
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
