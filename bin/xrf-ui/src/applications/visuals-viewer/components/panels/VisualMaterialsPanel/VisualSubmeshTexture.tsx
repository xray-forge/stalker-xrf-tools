import { Chip } from "@mui/material";
import { ReactElement } from "react";

import {
  describeResolution,
  describeTextureState,
  IVisualTextureStateDescriptor,
} from "@/applications/visuals-viewer/components/panels/VisualMaterialsPanel/VisualSubmeshTexture.utils";
import { VisualPanelRow } from "@/applications/visuals-viewer/components/panels/VisualPanelRow";
import { SubmeshTexture } from "@/core/bindings/xrf-app-visuals";
import { EVisualTextureState, IVisualTextureStatus } from "@/core/visuals/lib/visual-texture";
import { Nullable } from "@/lib/types/general";

export interface IVisualSubmeshTextureProps {
  texture: Nullable<SubmeshTexture>;
  status: Nullable<IVisualTextureStatus>;
}

/**
 * What became of one submesh's texture: the outcome, the root that answered, and the file inside it.
 *
 * Root and file are reported separately because they answer different questions - which tree the viewer resolved
 * against, and what it found there. With overlay trees the first is the one that explains a surprise.
 */
export function VisualSubmeshTexture({ texture, status }: IVisualSubmeshTextureProps): ReactElement | null {
  if (!texture) {
    return null;
  }

  const { resolution } = texture;
  const state: EVisualTextureState = status?.state ?? EVisualTextureState.ABSENT;
  const descriptor: IVisualTextureStateDescriptor = describeTextureState(state);

  return (
    <>
      <VisualPanelRow
        label={"Texture"}
        value={<Chip size={"small"} color={descriptor.color} variant={"outlined"} label={descriptor.label} />}
      />
      <VisualPanelRow label={"Resolution"} value={describeResolution(resolution)} />

      {resolution.kind === "resolved" || resolution.kind === "substituted" ? (
        <>
          <VisualPanelRow label={"Root"} value={resolution.location.root} />
          <VisualPanelRow label={"File"} value={resolution.location.relativePath} />
        </>
      ) : null}

      {resolution.kind === "missing" ? <VisualPanelRow label={"Root"} value={resolution.root} /> : null}

      {status?.reason ? <VisualPanelRow label={"Texture error"} value={status.reason} /> : null}
    </>
  );
}
