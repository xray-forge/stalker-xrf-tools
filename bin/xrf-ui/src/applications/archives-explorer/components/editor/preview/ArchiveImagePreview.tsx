import { Box, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivePreviewError } from "@/applications/archives-explorer/components/editor/preview/ArchivePreviewError";
import { ArchivesService } from "@/applications/archives-explorer/services/archives";
import { TArchiveContent } from "@/core/archive";
import { ArchiveImagePreview as TArchiveImagePreview } from "@/core/bindings/xrf-app";
import { DelayedProgress } from "@/core/ui/layout/DelayedProgress";
import { EmptyState } from "@/core/ui/layout/EmptyState";
import { ImageViewport } from "@/core/ui/media/ImageViewport";
import { Loadable } from "@/lib/loadable";
import { Nullable } from "@/lib/types/general";

/**
 * Shows an archived texture the backend decoded into a PNG.
 *
 * The checkerboard is the point rather than decoration: these are mostly UI textures with an alpha
 * channel, and on a flat background transparent regions are indistinguishable from black ones.
 */
export function ArchiveImagePreview(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;
  const preview: Nullable<TArchiveImagePreview> = content.value?.kind === "image" ? content.value.preview : null;

  if (content.isLoading) {
    return <DelayedProgress />;
  } else if (content.error) {
    return <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />;
  }

  return preview ? (
    <Box sx={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0, minHeight: 0 }}>
      <ImageViewport
        alt={preview.name}
        src={`data:image/png;base64,${preview.base64}`}
        width={preview.width}
        height={preview.height}
      />

      <Box sx={{ flexShrink: 0, paddingX: 1.5, paddingY: 0.5, borderTop: 1, borderColor: "divider" }}>
        <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
          {preview.width} x {preview.height}
        </Typography>
      </Box>
    </Box>
  ) : (
    <EmptyState title={"Preview unavailable"} description={"This texture could not be decoded."} />
  );
}
