import { Box, Divider, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useEffect, useState } from "react";

import { ArchiveFileDetailRow } from "@/applications/archive-editor/components/editor/file-details/ArchiveFileDetailRow";
import { ArchivePreviewError } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewError";
import { ArchivePreviewState } from "@/applications/archive-editor/components/editor/preview/ArchivePreviewState";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { CenteredColumn } from "@/core/components/layout/CenteredColumn";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { AssetService } from "@/core/store/assets";
import { Nullable } from "@/core/types/general";
import { IArchiveAudioPreview, TArchiveContent } from "@/lib/archive";
import { Loadable } from "@/lib/loadable";
import { base64ToBlob } from "@/lib/media/base64";

/** One sound is previewed at a time, so its url lives under a fixed key and displaces the last one. */
const ARCHIVE_AUDIO_ASSET_KEY: string = "archive-audio";

/**
 * Plays an archived sound and reports what the engine would read from it.
 */
export function ArchiveAudioPreview(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);
  const assetService: AssetService = useInjection(AssetService);

  const [url, setUrl] = useState<Nullable<string>>(null);

  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;
  const preview: Nullable<IArchiveAudioPreview> = content.value?.kind === "audio" ? content.value.preview : null;

  useEffect(() => {
    setUrl(preview ? assetService.swap(ARCHIVE_AUDIO_ASSET_KEY, base64ToBlob(preview.base64, "audio/ogg")) : null);
  }, [assetService, preview]);

  if (content.isLoading) {
    return <DelayedProgress />;
  }

  if (content.error) {
    return <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />;
  }

  if (!preview || !url) {
    return <ArchivePreviewState title={"Preview unavailable"} description={"This sound could not be read."} />;
  }

  return (
    <CenteredColumn sx={{ padding: 3, gap: 2 }}>
      <Box component={"audio"} controls={true} src={url} sx={{ width: "100%", maxWidth: 480 }} />

      <Box sx={{ width: "100%", maxWidth: 480 }}>
        <Typography variant={"subtitle2"}>Stream</Typography>

        <ArchiveFileDetailRow label={"Channels"} value={preview.channels ? String(preview.channels) : "-"} />
        <ArchiveFileDetailRow label={"Sample rate"} value={preview.sampleRate ? `${preview.sampleRate} Hz` : "-"} />

        <Divider sx={{ marginY: 1.5 }} />

        <Typography variant={"subtitle2"}>Engine parameters</Typography>

        {preview.parameters ? (
          <>
            <ArchiveFileDetailRow label={"Min distance"} value={String(preview.parameters.minDistance)} />
            <ArchiveFileDetailRow label={"Max distance"} value={String(preview.parameters.maxDistance)} />
            <ArchiveFileDetailRow label={"Base volume"} value={String(preview.parameters.baseVolume)} />
            <ArchiveFileDetailRow label={"Max AI distance"} value={String(preview.parameters.maxAiDistance)} />
            <ArchiveFileDetailRow label={"Game type"} value={String(preview.parameters.gameType)} mono />
          </>
        ) : (
          <Typography variant={"body2"} sx={{ marginTop: 1, color: "text.secondary" }}>
            This sound carries no X-Ray comment, so the engine would use its built-in source defaults.
          </Typography>
        )}
      </Box>
    </CenteredColumn>
  );
}
