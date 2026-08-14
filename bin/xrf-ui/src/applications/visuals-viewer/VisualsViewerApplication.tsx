import { ReactElement } from "react";

import { VisualPreviewLayout } from "@/core/visuals/preview/VisualPreviewLayout";

/**
 * Single visual mode: one model opened directly, with no project context around it.
 */
export function VisualsViewerApplication(): ReactElement {
  return <VisualPreviewLayout />;
}
