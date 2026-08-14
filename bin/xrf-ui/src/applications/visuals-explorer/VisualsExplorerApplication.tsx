import { ReactElement } from "react";

import { VisualProjectTree } from "@/applications/visuals-explorer/components/VisualProjectTree";
import { VisualPreviewLayout } from "@/core/visuals/preview/VisualPreviewLayout";

/**
 * Project mode: browse a gamedata or resources tree and open visuals from it.
 */
export function VisualsExplorerApplication(): ReactElement {
  return <VisualPreviewLayout tree={<VisualProjectTree />} />;
}
