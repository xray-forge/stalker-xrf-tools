import { Box, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivePreviewError } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewError";
import { ArchivePreviewState } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewState";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { Nullable } from "@/core/types/general";
import { IArchiveImagePreview, TArchiveContent } from "@/lib/archive";
import { Loadable } from "@/lib/loadable";

/**
 * Shows an archived texture the backend decoded into a PNG.
 *
 * The checkerboard is the point rather than decoration: these are mostly UI textures with an alpha
 * channel, and on a flat background transparent regions are indistinguishable from black ones.
 */
export function ArchiveImagePreview(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;
  const preview: Nullable<IArchiveImagePreview> = content.value?.kind === "image" ? content.value.preview : null;

  if (content.isLoading) {
    return <DelayedProgress />;
  } else if (content.error) {
    return <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />;
  }

  return preview ? (
    <Box sx={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0, minHeight: 0 }}>
      <Box
        sx={{
          display: "flex",
          flexGrow: 1,
          minHeight: 0,
          alignItems: "center",
          justifyContent: "center",
          overflow: "auto",
          padding: 2,
          backgroundColor: "#353535",
          backgroundImage: [
            "linear-gradient(45deg, #707070 25%, transparent 25%)",
            "linear-gradient(-45deg, #808080 25%, transparent 25%)",
            "linear-gradient(45deg, transparent 75%, #808080 75%)",
            "linear-gradient(-45deg, transparent 75%, #808080 75%)",
          ].join(","),
          backgroundSize: "20px 20px",
          backgroundPosition: "0 0, 0 10px, 10px -10px, -10px 0px",
        }}
      >
        <Box
          component={"img"}
          alt={preview.name}
          src={`data:image/png;base64,${preview.base64}`}
          sx={{ maxWidth: "100%", maxHeight: "100%", imageRendering: "pixelated", objectFit: "contain" }}
        />
      </Box>

      <Box sx={{ flexShrink: 0, paddingX: 1.5, paddingY: 0.5, borderTop: 1, borderColor: "divider" }}>
        <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
          {preview.width} x {preview.height}
        </Typography>
      </Box>
    </Box>
  ) : (
    <ArchivePreviewState title={"Preview unavailable"} description={"This texture could not be decoded."} />
  );
}
