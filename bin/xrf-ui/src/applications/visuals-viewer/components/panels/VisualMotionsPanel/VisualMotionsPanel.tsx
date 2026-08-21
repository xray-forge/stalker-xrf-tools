import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { VisualMotionNames } from "@/applications/visuals-viewer/components/panels/VisualMotionsPanel/VisualMotionNames";
import { VisualPanel } from "@/applications/visuals-viewer/components/panels/VisualPanel";
import { VisualPanelEmpty } from "@/applications/visuals-viewer/components/panels/VisualPanelEmpty";
import { VisualPanelSection } from "@/applications/visuals-viewer/components/panels/VisualPanelSection";
import { VisualsService } from "@/applications/visuals-viewer/store/visuals";
import { VisualDescription } from "@/core/bindings/types/xrf-visual";
import { Nullable } from "@/lib/types/general";

/**
 * What this visual can animate from, listed but not playable.
 *
 * Referenced omf files are named rather than resolved: finding them needs a gamedata root, which the application does
 * not model yet, and playback is a later phase either way.
 */
export function VisualMotionsPanel(): ReactElement {
  const visualsService: VisualsService = useInjection(VisualsService);
  const description: Nullable<VisualDescription> = visualsService.visual.value?.selected.description ?? null;

  const refs: Array<string> = description?.motionRefs ?? [];
  const embedded: Array<string> = description?.embeddedMotions ?? [];

  if (refs.length === 0 && embedded.length === 0) {
    return (
      <VisualPanel title={"Motions"}>
        <VisualPanelEmpty label={"No motions. Resolved from the visual's omf motion refs."} />
      </VisualPanel>
    );
  }

  return (
    <VisualPanel title={"Motions"}>
      {refs.length > 0 ? (
        <VisualPanelSection title={`Motion refs (${refs.length})`} caption={"Omf files the engine loads"} isFirst>
          <VisualMotionNames names={refs} />
        </VisualPanelSection>
      ) : null}

      {embedded.length > 0 ? (
        <VisualPanelSection
          title={`Embedded motions (${embedded.length})`}
          caption={"Stored inside this visual"}
          isFirst={refs.length === 0}
        >
          <VisualMotionNames names={embedded} />
        </VisualPanelSection>
      ) : null}
    </VisualPanel>
  );
}
