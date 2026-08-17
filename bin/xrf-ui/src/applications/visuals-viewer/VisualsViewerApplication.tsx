import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useState } from "react";

import { VISUAL_VIEWER_PANELS } from "@/applications/visuals-viewer/components/panels/visual-viewer-panels";
import { VisualsViewerOpenForm } from "@/applications/visuals-viewer/components/VisualsViewerOpenForm";
import { IOpenVisual, VisualsService } from "@/applications/visuals-viewer/store/visuals";
import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";
import { VisualPreviewLayout } from "@/core/visuals/preview/VisualPreviewLayout";
import { Nullable } from "@/lib/types/general";

/**
 * Single visual mode: one model opened directly, with no project context around it.
 */
export function VisualsViewerApplication(): ReactElement {
  const visualsService: VisualsService = useInjection(VisualsService);
  const [isPickerOpen, setPickerOpen] = useState(false);

  const visual: Nullable<IOpenVisual> = visualsService.visual.value;

  const onOpen = useCallback(() => setPickerOpen(true), []);

  const onFinished = useCallback(() => setPickerOpen(false), []);

  if (!visualsService.isReady || visualsService.visual.isLoading) {
    return <ApplicationLoader />;
  }

  if (!visual || isPickerOpen) {
    return <VisualsViewerOpenForm onFinished={onFinished} />;
  }

  return (
    <VisualPreviewLayout
      model={visual.views}
      subtitle={visualsService.sourceLabel ?? undefined}
      panels={VISUAL_VIEWER_PANELS}
      onOpen={onOpen}
    />
  );
}
