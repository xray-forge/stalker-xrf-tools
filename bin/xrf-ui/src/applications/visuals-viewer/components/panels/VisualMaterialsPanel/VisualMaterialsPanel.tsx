import { useInjection } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { VisualSubmeshSection } from "@/applications/visuals-viewer/components/panels/VisualMaterialsPanel/VisualSubmeshSection";
import { VisualPanel } from "@/applications/visuals-viewer/components/panels/VisualPanel";
import { VisualPanelEmpty } from "@/applications/visuals-viewer/components/panels/VisualPanelEmpty";
import { VisualsService } from "@/applications/visuals-viewer/store/visuals";
import { SubmeshTexture } from "@/core/bindings/types/xrf-app";
import { VisualDescription } from "@/core/bindings/types/xrf-visual";
import { Nullable } from "@/lib/types/general";

/** Every submesh of the open visual, in the order the file stores them. */
export function VisualMaterialsPanel(): ReactElement {
  const visualsService: VisualsService = useInjection(VisualsService);
  const description: Nullable<VisualDescription> = visualsService.visual.value?.selected.description ?? null;

  const textures: ReadonlyMap<number, SubmeshTexture> = useMemo(
    () => new Map((visualsService.visual.value?.selected.textures ?? []).map((it) => [it.submeshIndex, it])),
    [visualsService.visual.value]
  );

  if (!description || description.submeshes.length === 0) {
    return (
      <VisualPanel title={"Materials"}>
        <VisualPanelEmpty label={"No materials. Texture and shader names per child visual."} />
      </VisualPanel>
    );
  }

  return (
    <VisualPanel title={"Materials"}>
      {description.submeshes.map((submesh, index) => (
        <VisualSubmeshSection
          key={submesh.index}
          submesh={submesh}
          isFirst={index === 0}
          texture={textures.get(submesh.index) ?? null}
          status={visualsService.textureStatuses.get(submesh.index) ?? null}
        />
      ))}
    </VisualPanel>
  );
}
