import { Box, Divider, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useMemo, useState } from "react";

import { ArchiveFileDetailRow } from "@/applications/archives/components/editor/file-details/ArchiveFileDetailRow";
import { ArchivePreviewError } from "@/applications/archives/components/editor/preview/ArchivePreviewError";
import { ArchivesService } from "@/applications/archives/services/archives";
import { TArchiveContent } from "@/core/archive";
import { AssetService } from "@/core/assets/services";
import { CenteredColumn } from "@/core/components/layout/CenteredColumn";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { EmptyState } from "@/core/components/layout/EmptyState";
import { AudioPlayer } from "@/core/components/media/AudioPlayer";
import { Loadable } from "@/lib/loadable";
import { base64ToBytes } from "@/lib/media/base64";
import { Nullable } from "@/lib/types/general";
import { ArchiveAudioPreview as TArchiveAudioPreview } from "@/lib/xrf/bindings/xrf-app";

/** One sound is previewed at a time, so its url lives under a fixed key and displaces the last one. */
const ARCHIVE_AUDIO_ASSET_KEY: string = "archive-audio";

/** Wide enough for a waveform to be readable, narrow enough that the detail rows stay scannable. */
const ARCHIVE_AUDIO_PREVIEW_WIDTH: number = 640;

/**
 * Plays an archived sound and reports what the engine would read from it.
 */
export function ArchiveAudioPreview(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);
  const assetService: AssetService = useInjection(AssetService);

  const [url, setUrl] = useState<Nullable<string>>(null);

  const content: Loadable<Nullable<TArchiveContent>> = archivesService.content;
  const preview: Nullable<TArchiveAudioPreview> = content.value?.kind === "audio" ? content.value.preview : null;

  // Decoded once and shared: the element streams from the url, the waveform reads the same bytes.
  const bytes: Nullable<Uint8Array> = useMemo(() => (preview ? base64ToBytes(preview.base64) : null), [preview]);

  const formatChannels = useCallback((channels: number): string => {
    switch (channels) {
      case 0:
        return "-";

      case 1:
        return "1 (mono)";

      case 2:
        return "2 (stereo)";

      default:
        return String(channels);
    }
  }, []);

  useEffect(() => {
    const blob: Nullable<Blob> = bytes ? new Blob([bytes.buffer as ArrayBuffer], { type: "audio/ogg" }) : null;

    setUrl(blob ? assetService.swap(ARCHIVE_AUDIO_ASSET_KEY, blob) : null);
  }, [assetService, bytes]);

  if (content.isLoading) {
    return <DelayedProgress />;
  }

  if (content.error) {
    return <ArchivePreviewError error={content.error} onRetry={archivesService.retrySelectedFile} />;
  }

  if (!preview || !url) {
    return <EmptyState title={"Preview unavailable"} description={"This sound could not be read."} />;
  }

  return (
    <CenteredColumn
      sx={{
        padding: 3,
        gap: 2.5,
        overflowY: "auto",
        justifyContent: "safe center",
      }}
    >
      <Box sx={{ flexShrink: 0, width: "100%", maxWidth: ARCHIVE_AUDIO_PREVIEW_WIDTH }}>
        <AudioPlayer src={url} bytes={bytes} />
      </Box>

      <Box sx={{ flexShrink: 0, width: "100%", maxWidth: ARCHIVE_AUDIO_PREVIEW_WIDTH }}>
        <Typography variant={"subtitle2"}>Stream</Typography>

        <ArchiveFileDetailRow label={"Channels"} value={formatChannels(preview.channels)} />
        <ArchiveFileDetailRow label={"Sample rate"} value={preview.sampleRate ? `${preview.sampleRate} Hz` : "-"} />

        <Divider sx={{ marginY: 1.5 }} />

        <Typography variant={"subtitle2"}>Engine parameters</Typography>

        {preview.parameters ? (
          <>
            <ArchiveFileDetailRow label={"Min distance"} value={`${preview.parameters.minDistance} m`} />
            <ArchiveFileDetailRow label={"Max distance"} value={`${preview.parameters.maxDistance} m`} />
            <ArchiveFileDetailRow label={"Max AI distance"} value={`${preview.parameters.maxAiDistance} m`} />
            <ArchiveFileDetailRow
              label={"Base volume"}
              value={`${preview.parameters.baseVolume ?? 0} (${Math.round(
                (preview.parameters.baseVolume ?? 0) * 100
              )}%)`}
            />
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
