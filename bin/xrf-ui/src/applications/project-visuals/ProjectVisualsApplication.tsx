import { ReactElement } from "react";

import { VisualProjectTree } from "@/applications/project-visuals/components/VisualProjectTree";
import { VisualPreviewLayout } from "@/lib/visuals/preview/VisualPreviewLayout";

/**
 * Project mode: browse a gamedata or resources tree and open visuals from it.
 */
export function ProjectVisualsApplication(): ReactElement {
  return <VisualPreviewLayout tree={<VisualProjectTree />} />;
}
