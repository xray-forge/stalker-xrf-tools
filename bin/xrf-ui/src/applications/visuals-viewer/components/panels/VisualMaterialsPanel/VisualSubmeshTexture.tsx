import { Chip } from "@mui/material";
import { ReactElement } from "react";

import {
  describeResolution,
  describeTextureState,
  IVisualTextureStateDescriptor,
} from "@/applications/visuals-viewer/components/panels/VisualMaterialsPanel/VisualSubmeshTexture.utils";
import { VisualSubmeshTextureSource } from "@/applications/visuals-viewer/components/panels/VisualMaterialsPanel/VisualSubmeshTextureSource";
import { VisualPanelRow } from "@/applications/visuals-viewer/components/panels/VisualPanelRow";
import { XrayAsset } from "@/core/bindings/types/xrf-vfs";
import { VisualTextureDependency } from "@/core/bindings/types/xrf-visual";
import { EVisualTextureState, getLocatedAsset, IVisualTextureStatus } from "@/core/visuals/lib/visual-texture";
import { Nullable } from "@/lib/types/general";

export interface IVisualSubmeshTextureProps {
  texture: Nullable<VisualTextureDependency>;
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
  const located: Nullable<XrayAsset> = getLocatedAsset(resolution);
  const state: EVisualTextureState = status?.state ?? EVisualTextureState.ABSENT;
  const descriptor: IVisualTextureStateDescriptor = describeTextureState(state);

  return (
    <>
      <VisualPanelRow
        label={"Texture"}
        value={<Chip size={"small"} color={descriptor.color} variant={"outlined"} label={descriptor.label} />}
      />
      <VisualPanelRow label={"Resolution"} value={describeResolution(resolution)} />

      {located ? <VisualSubmeshTextureSource container={located.container} /> : null}

      {resolution.kind === "missing"
        ? resolution.roots.map((root: string) => <VisualPanelRow key={root} label={"Searched"} value={root} />)
        : null}

      {resolution.kind === "rejected" ? <VisualPanelRow label={"Rejected"} value={resolution.reason} /> : null}

      {status?.reason ? <VisualPanelRow label={"Texture error"} value={status.reason} /> : null}
    </>
  );
}
