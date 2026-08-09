import { ReactElement } from "react";

import { VisualPreviewLayout } from "@/applications/visuals_editor/components/preview/VisualPreviewLayout";

/**
 * Single visual mode: one model opened directly, with no project context around it.
 */
export function VisualsEditorPreviewPage(): ReactElement {
  return <VisualPreviewLayout />;
}
