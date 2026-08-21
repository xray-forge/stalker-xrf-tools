import { ReactElement } from "react";

import { VisualPanelRow } from "@/applications/visuals-viewer/components/panels/VisualPanelRow";
import { VisualPanelSection } from "@/applications/visuals-viewer/components/panels/VisualPanelSection";
import { VisualBounds } from "@/core/bindings/types/xrf-visual";
import { formatCoordinate, formatVector } from "@/core/visuals/lib/visual-format";
import { Nullable } from "@/lib/types/general";

export interface IVisualBoundsSectionProps {
  title: string;
  /** Which of the two extents this is, since the rows themselves are identical. */
  caption: string;
  bounds: Nullable<VisualBounds>;
}

/** One extent, as a box and the sphere around it. */
export function VisualBoundsSection({ title, caption, bounds }: IVisualBoundsSectionProps): ReactElement {
  return (
    <VisualPanelSection title={title} caption={caption}>
      <VisualPanelRow label={"Min"} value={formatVector(bounds?.boundingBox.min ?? null)} />
      <VisualPanelRow label={"Max"} value={formatVector(bounds?.boundingBox.max ?? null)} />
      <VisualPanelRow label={"Centre"} value={formatVector(bounds?.boundingSphere.center ?? null)} />
      <VisualPanelRow label={"Radius"} value={formatCoordinate(bounds?.boundingSphere.radius ?? null)} />
    </VisualPanelSection>
  );
}
