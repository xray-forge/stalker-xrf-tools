import { ReactElement } from "react";

import { VisualPreviewLayout } from "@/lib/visuals/preview/VisualPreviewLayout";

/**
 * Single visual mode: one model opened directly, with no project context around it.
 */
export function VisualPreviewApplication(): ReactElement {
  return <VisualPreviewLayout />;
}
