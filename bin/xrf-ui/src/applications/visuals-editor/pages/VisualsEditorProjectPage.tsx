import { ReactElement } from "react";

import { VisualPreviewLayout } from "@/applications/visuals-editor/components/preview/VisualPreviewLayout";
import { VisualProjectTree } from "@/applications/visuals-editor/components/preview/VisualProjectTree";

/**
 * Project mode: browse a gamedata or resources tree and open visuals from it.
 */
export function VisualsEditorProjectPage(): ReactElement {
  return <VisualPreviewLayout tree={<VisualProjectTree />} />;
}
